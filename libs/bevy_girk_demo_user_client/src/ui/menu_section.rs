//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_kot::prelude::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn activate_new_selection<U: LunexUI>(
    In((newly_selected_button, overlay)) : In<(Entity, Widget)>,
    mut ui                               : UiUtils<U>,
    selected_main_menu_button            : Query<(Entity, &Callback<Deselect>), (With<MainMenuButton>, With<Selected>)>,
){
    // activate the selected button's overlay
    //todo: use overlay widget's embedded entity
    ui.toggle(true, &overlay);

    // deselect any selected main menu buttons other than the newly selected button
    for (entity, deselect_callback) in selected_main_menu_button.iter()
    {
        if entity == newly_selected_button { continue; }
        ui.builder.rcommands.commands().add(deselect_callback.clone());
    }
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
                ui.asset_server.load(MENU_BAR_BUTTON.0),
                MENU_BAR_BUTTON.1
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
            ui.asset_server.load(MENU_BAR_BUTTON.0),
            MENU_BAR_BUTTON.1
        );
    ui.commands().spawn(selected_image);

    // build the button
    let mut entity_commands = ui.commands().spawn_empty();
    let button_entity = entity_commands.id();
    let overlay_clone = overlay.clone();

    InteractiveElementBuilder::new()
        .with_default_widget(default_button)
        .with_selected_widget(selected_button)
        .select_on_click()
        .on_select(
            prep_syscall((button_entity, overlay.clone()), activate_new_selection::<MainUI>)
        )
        .on_deselect(
            move |mut ui: UiUtils<MainUI>| ui.toggle(false, &overlay_clone)
        )
        .build::<MouseLButtonMain>(&mut entity_commands, button.clone())
        .unwrap();

    // add main menu button tag
    entity_commands.insert(MainMenuButton);

    // set text style
    ui.add_style(basic_text_default_light_style());

    // add button text
    let text = make_overlay(ui.tree(), button, "", true);
    spawn_basic_text(
            ui,
            text,
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
    // set text style
    ui.add_style(basic_text_default_light_style());

    // spawn text
    let text = relative_widget(ui.tree(), area.end(""), (40., 60.), (40., 60.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(40.)), "Welcome!");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_settings_overlay(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // spawn text
    let text = relative_widget(ui.tree(), area.end(""), (40., 60.), (40., 60.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(20.)), "There are no settings yet...");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct MainMenuButton;

pub(crate) fn add_menu_bar_section(ui: &mut UiBuilder<MainUI>, menu_bar: &Widget, menu_overlay: &Widget)
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
    let home_overlay = make_overlay(ui.tree(), menu_overlay, "home_overlay", false);
    let home_button_entity =
    ui.div(|ui| add_menu_bar_button(ui, &menu_widgets[0], &home_overlay, "HOME"));
    ui.div(|ui| add_home_overlay(ui, &home_overlay));

    // - settings
    let settings_overlay = make_overlay(ui.tree(), menu_overlay, "settings_overlay", false);
    let _ =
    ui.div(|ui| add_menu_bar_button(ui, &menu_widgets[1], &settings_overlay, "SETTINGS"));
    ui.div(|ui| add_settings_overlay(ui, &settings_overlay));

    // activate home button (default)
    ui.commands().add(move |world: &mut World| { let _ = try_callback::<Select>(world, home_button_entity); } );
}

//-------------------------------------------------------------------------------------------------------------------
