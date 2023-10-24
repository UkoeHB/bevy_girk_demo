//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_kot::ecs::{*, syscall};
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn activate_new_selection<U: LunexUI>(
    In((newly_selected_button, overlay)) : In<(Entity, Widget)>,
    mut commands                         : Commands,
    mut uis                              : Query<&mut UiTree, With<U>>,
    selected_main_menu_button            : Query<(Entity, &Callback<Deselect>), (With<MainMenuButton>, With<Selected>)>,
){
    // activate the selected button's overlay
    let mut ui = uis.get_single_mut().unwrap();  //todo: use overlay widget embedded entity
    if let Ok(overlay_branch) = overlay.fetch_mut(&mut ui) { overlay_branch.set_visibility(true); }

    // deselect any selected main menu buttons other than the newly selected button
    for (entity, deselect_callback) in selected_main_menu_button.iter()
    {
        if entity == newly_selected_button { continue; }
        commands.add(deselect_callback.clone());
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn deactivate_selection<U: LunexUI>(
    In(overlay) : In<Widget>,
    mut uis     : Query<&mut UiTree, With<U>>,
){
    // deactivate the overlay of a deselected menu button
    let mut ui = uis.get_single_mut().unwrap();  //todo: use overlay widget embedded entity
    if let Ok(overlay_branch) = overlay.fetch_mut(&mut ui) { overlay_branch.set_visibility(false); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_menu_bar_button(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    button       : &Widget,
    overlay      : &Widget,
    asset_server : &AssetServer,
    display_name : &str,
) -> Entity
{
    // add default button image
    let default_button = make_overlay(ui, button, "default", true);
    commands.spawn(
        ImageElementBundle::new(
                &default_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.)),
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add selected button image
    let selected_button = make_overlay(ui, button, "selected", false);
    commands.spawn(
        ImageElementBundle::new(
                &selected_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::GRAY),  //tint the default button (todo: it's ugly)
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // select callback
    let mut entity_commands = commands.spawn_empty();
    let button_entity = entity_commands.id();
    let overlay_clone = overlay.clone();
    let select_callback =
        move |world: &mut World|
        {
            syscall(world, (button_entity, overlay_clone.clone()), activate_new_selection::<MainUI>);
        };
    let overlay_clone = overlay.clone();
    let deselect_callback =
        move |world: &mut World|
        {
            syscall(world, overlay_clone.clone(), deactivate_selection::<MainUI>);
        };

    // build interaction into the widget
    InteractiveElementBuilder::new()
        .with_default_widget(default_button)
        .with_selected_widget(selected_button)
        .select_on_click()
        .select_callback(select_callback)
        .deselect_callback(deselect_callback)
        .build::<MouseLButtonMain>(&mut entity_commands, button.clone())
        .unwrap();

    // add main menu button tag
    entity_commands.insert(MainMenuButton);

    // add button text
    let menu_bar_text_style = TextStyle{
            font      : asset_server.load(MENU_BAR_BUTTON_FONT),
            font_size : 40.0,
            color     : MENU_BAR_BUTTON_FONT_COLOR,
        };

    entity_commands.insert(
        TextElementBundle::new(
                button,
                TextParams::center()
                    .with_style(&menu_bar_text_style)
                    .with_depth(100.)
                    .with_height(Some(40.)),
                display_name
            )
    );

    button_entity
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_home_overlay(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    overlay      : &Widget,
    asset_server : &AssetServer
){
    // overlay
    let home_overlay_text = Widget::create(
            ui,
            overlay.end(""),
            RelativeLayout{  //center
                relative_1: Vec2{ x: 40., y: 40. },
                relative_2: Vec2{ x: 60., y: 60. },
                ..Default::default()
            }
        ).unwrap();

    let home_overlay_text_style = TextStyle {
            font      : asset_server.load(TEMP_FONT),
            font_size : 45.0,
            color     : TEMP_FONT_COLOR,
        };

    commands.spawn(
            TextElementBundle::new(
                home_overlay_text,
                TextParams::center()
                    .with_style(&home_overlay_text_style)
                    .with_height(Some(40.)),
                "Welcome!"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_settings_overlay(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    overlay      : &Widget,
    asset_server : &AssetServer
){
    // overlay
    let settings_overlay_text = Widget::create(
            ui,
            overlay.end(""),
            RelativeLayout{  //center
                relative_1: Vec2{ x: 40., y: 40. },
                relative_2: Vec2{ x: 60., y: 60. },
                ..Default::default()
            }
        ).unwrap();

    let settings_overlay_text_style = TextStyle {
            font      : asset_server.load(TEMP_FONT),
            font_size : 45.0,
            color     : TEMP_FONT_COLOR,
        };

    commands.spawn(
            TextElementBundle::new(
                settings_overlay_text,
                TextParams::center()
                    .with_style(&settings_overlay_text_style)
                    .with_height(Some(20.)),
                "There are no settings yet..."
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct MainMenuButton;

pub(crate) fn add_menu_bar_section(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    menu_bar     : Widget,
    menu_overlay : Widget,
){
    // menu bar overlay
    let menu_bar_overlay = Widget::create(
            ui,
            menu_bar.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 20. },
                relative_2: Vec2{ x: 90., y: 100. },
                ..Default::default()
            }
        ).unwrap();

    // menu buttons: widget grid
    let menu_widgets = GridSegment::new()
        .with_cells(vec![GridCell::named(Vec2::splat(10.0), "home"), GridCell::named(Vec2::splat(10.0), "settings")])
        .add_gaps(1.0)
        .build_in(ui, &menu_bar_overlay, GridOrientation::Horizontal)
        .unwrap();

    // prepare each of the menu buttons and areas
    // - home
    let home_overlay = make_overlay(ui, &menu_overlay, "home_overlay", false);
    let home_button_entity = add_menu_bar_button(commands, ui, &menu_widgets[0], &home_overlay, asset_server, "HOME");
    add_home_overlay(commands, ui, &home_overlay, asset_server);

    // - settings
    let settings_overlay = make_overlay(ui, &menu_overlay, "settings_overlay", false);
    let _ = add_menu_bar_button(commands, ui, &menu_widgets[1], &settings_overlay, asset_server, "SETTINGS");
    add_settings_overlay(commands, ui, &settings_overlay, asset_server);

    // activate home button (default)
    commands.add(move |world: &mut World| { let _ = try_callback::<Select>(world, home_button_entity); } );
}

//-------------------------------------------------------------------------------------------------------------------
