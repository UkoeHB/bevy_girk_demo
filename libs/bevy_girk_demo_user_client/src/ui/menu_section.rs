//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_kot::prelude::{*, builtin::*};
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
    let mut ui = uis.get_single_mut().unwrap();  //todo: use overlay widget's embedded entity
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

fn add_menu_bar_button(ui: &mut UiBuilder<MainUI>, button: &Widget, overlay: &Widget, display_name: &str) -> Entity
{
    // add default button image
    let default_button = make_overlay(ui.tree(), button, "default", true);
    let default_image = ImageElementBundle::new(
            &default_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ui.asset_server.load(MENU_BAR_BUTTON),
            Vec2::new(250.0, 142.0)
        );
    ui.commands().spawn(default_image);

    // add selected button image
    let selected_button = make_overlay(ui.tree(), button, "selected", false);
    let selected_image = ImageElementBundle::new(
            &selected_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(Color::GRAY),  //tint the default button (todo: it's ugly)
            ui.asset_server.load(MENU_BAR_BUTTON),
            Vec2::new(250.0, 142.0)
        );
    ui.commands().spawn(selected_image);

    // select callback
    let mut entity_commands = ui.commands().spawn_empty();
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

    // build the button
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
    let text = make_overlay(ui.tree(), button, "", true);
    spawn_basic_text(
            ui,
            text,
            MENU_BAR_BUTTON_FONT_COLOR,
            TextParams::center()
                .with_depth(100.)
                .with_height(Some(40.)),
            display_name
        );

    button_entity
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_home_overlay(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    let text = relative_widget(ui.tree(), area.end(""), (40., 60.), (40., 60.));
    spawn_basic_text(
            ui,
            text,
            MISC_FONT_COLOR,
            TextParams::center()
                .with_height(Some(40.)),
            "Welcome!"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_settings_overlay(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    let text = relative_widget(ui.tree(), area.end(""), (40., 60.), (40., 60.));
    spawn_basic_text(
            ui,
            text,
            MISC_FONT_COLOR,
            TextParams::center()
                .with_height(Some(20.)),
            "There are no settings yet..."
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct MainMenuButton;

pub(crate) fn add_menu_bar_section(ui: &mut UiBuilder<MainUI>, menu_bar: Widget, menu_overlay: Widget)
{
    // menu bar overlay
    let menu_bar_overlay = relative_widget(ui.tree(), menu_bar.end(""), (10., 90.), (10., 90.));

    // menu buttons: widget grid
    let menu_widgets = GridSegment::new()
        .with_cells(vec![GridCell::named(Vec2::splat(10.0), "home"), GridCell::named(Vec2::splat(10.0), "settings")])
        .add_gaps(1.0)
        .build_in(ui.tree(), &menu_bar_overlay, GridOrientation::Horizontal)
        .unwrap();

    // prepare each of the menu buttons and areas
    // - home
    let home_overlay = make_overlay(ui.tree(), &menu_overlay, "home_overlay", false);
    let home_button_entity = add_menu_bar_button(ui, &menu_widgets[0], &home_overlay, "HOME");
    add_home_overlay(ui, &home_overlay);

    // - settings
    let settings_overlay = make_overlay(ui.tree(), &menu_overlay, "settings_overlay", false);
    let _ = add_menu_bar_button(ui, &menu_widgets[1], &settings_overlay, "SETTINGS");
    add_settings_overlay(ui, &settings_overlay);

    // activate home button (default)
    ui.commands().add(move |world: &mut World| { let _ = try_callback::<Select>(world, home_button_entity); } );
}

//-------------------------------------------------------------------------------------------------------------------
