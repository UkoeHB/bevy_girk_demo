use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn activate_new_selection<U: LunexUi>(
    In((newly_selected_button, overlay)): In<(Entity, Widget)>,
    mut ui: UiUtils<U>,
    selected_main_menu_button: Query<
        (Entity, &bevy_kot::prelude::Callback<Deselect>),
        (With<MainMenuButton>, With<Selected>),
    >,
)
{
    // activate the selected button's overlay
    //todo: use overlay widget's embedded entity
    ui.toggle(true, &overlay);

    // deselect any selected main menu buttons other than the newly selected button
    for (entity, deselect_callback) in selected_main_menu_button.iter() {
        if entity == newly_selected_button {
            continue;
        }
        ui.builder
            .rcommands
            .commands()
            .add(deselect_callback.clone());
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn add_menu_bar_button(ui: &mut UiBuilder<MainUi>, button: &Widget, overlay: &Widget, display_name: &str)
    -> Entity
{
    // button style
    let style = ui.style::<MenuButtonStyle>();
    ui.add_style(style.text.clone());

    // add default button image
    let default_button = make_overlay(ui.tree(), button, "default", true);
    let default_image = ImageElementBundle::new(
        &default_button,
        ImageParams::center()
            .with_depth(50.)
            .with_width(Some(100.))
            .with_height(Some(100.))
            .with_color(style.default_img_color),
        ui.asset_server.load(style.default_img.0),
        style.default_img.1,
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
            .with_color(style.pressed_img_color),
        ui.asset_server.load(style.pressed_img.0),
        style.pressed_img.1,
    );
    ui.commands().spawn(selected_image);

    // build the button
    let despawner = ui.despawner.clone();
    let mut entity_commands = ui.commands().spawn_empty();
    let button_entity = entity_commands.id();
    let overlay_clone = overlay.clone();

    InteractiveElementBuilder::new()
        .with_default_widget(default_button)
        .with_selected_widget(selected_button)
        .select_on_click()
        .on_select(prep_fncall(
            (button_entity, overlay.clone()),
            activate_new_selection::<MainUi>,
        ))
        .on_deselect(move |mut ui: UiUtils<MainUi>| ui.toggle(false, &overlay_clone))
        .build::<MouseLButtonMain>(&despawner, &mut entity_commands, button.clone())
        .unwrap();

    // add main menu button tag
    entity_commands.insert(MainMenuButton);

    // add button text
    let text = make_overlay(ui.tree(), button, "", true);
    spawn_basic_text(
        ui,
        text,
        TextParams::center().with_depth(100.).with_height(Some(40.)),
        display_name,
    );

    button_entity
}

//-------------------------------------------------------------------------------------------------------------------

fn add_home_overlay(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // spawn text
    let text = relative_widget(ui.tree(), area.end(""), (40., 60.), (40., 60.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(40.)), "Welcome!");
}

//-------------------------------------------------------------------------------------------------------------------

fn add_settings_overlay(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // spawn text
    let text = relative_widget(ui.tree(), area.end(""), (40., 60.), (40., 60.));
    spawn_basic_text(
        ui,
        text,
        TextParams::center().with_height(Some(20.)),
        "There are no settings yet...",
    );
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct MainMenuButton;

pub(crate) fn add_menu_bar_section(ui: &mut UiBuilder<MainUi>, menu_bar: &Widget, menu_overlay: &Widget)
{
    // menu bar overlay
    let menu_bar_overlay = relative_widget(ui.tree(), menu_bar.end(""), (10., 90.), (10., 90.));

    // menu buttons: widget grid
    let menu_widgets = GridSegment::new()
        .with_cells(
            vec![GridCell::named(Vec2::splat(10.0), "home"), GridCell::named(Vec2::splat(10.0), "settings")],
        )
        .add_gaps(1.0)
        .build_in(ui.tree(), &menu_bar_overlay, GridOrientation::Horizontal)
        .unwrap();

    // prepare each of the menu buttons and areas
    // - home
    let home_overlay = make_overlay(ui.tree(), menu_overlay, "home_overlay", false);
    let home_button_entity = ui.div(|ui| add_menu_bar_button(ui, &menu_widgets[0], &home_overlay, "HOME"));
    ui.div(|ui| add_home_overlay(ui, &home_overlay));

    // - settings
    let settings_overlay = make_overlay(ui.tree(), menu_overlay, "settings_overlay", false);
    let _ = ui.div(|ui| add_menu_bar_button(ui, &menu_widgets[1], &settings_overlay, "SETTINGS"));
    ui.div(|ui| add_settings_overlay(ui, &settings_overlay));

    // activate home button (default)
    ui.commands().add(move |world: &mut World| {
        let _ = try_callback::<Select>(world, home_button_entity);
    });
}

//-------------------------------------------------------------------------------------------------------------------

/// Resource that tracks which 'primary' section of the menu is visible.
//TODO: update this when changing sections
#[derive(Resource, Debug, Default)]
pub(crate) enum MenuContentSection
{
    #[default]
    Home,
    Play,
    Settings,
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct MenuContentPlugin;

impl Plugin for MenuContentPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_resource::<MenuContentSection>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
