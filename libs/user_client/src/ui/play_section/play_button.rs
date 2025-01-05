use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_kot_ui::builtin::{MainUi, MouseLButtonMain};
use bevy_kot_ui::{make_overlay, toggle_ui_visibility, Deselect, InteractiveElementBuilder, Selected, UiBuilder};
use bevy_lunex::prelude::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Try to set the value of a component on an entity.
///
/// Returns `true` if the component was added-to or updated on the entity.
/// - Returns `false` if the value wouldn't change.
/// - Does not trigger change detection unless the component is added or modified.
fn try_set_component<C: Component + Eq>(world: &mut World, entity: Entity, component: C) -> bool
{
    // try to get the entity
    let Some(mut entity_mut) = world.get_entity_mut(entity) else {
        return false;
    };

    // get or insert the value
    let Some(mut existing_component) = entity_mut.get_mut::<C>() else {
        entity_mut.insert(component);
        return true;
    };

    // update if value is new
    if *(existing_component.bypass_change_detection()) == component {
        return false;
    }
    *existing_component = component;
    true
}

//-------------------------------------------------------------------------------------------------------------------

fn set_play_button_state(
    mut commands: Commands,
    display: ReactRes<LobbyDisplay>,
    callback: Query<&CallbackWith<Toggle, MainPlayButton>>,
)
{
    let cb = callback.single();

    match display.is_set() {
        true => commands.add(cb.call_with(MainPlayButton::InLobby)),
        false => commands.add(cb.call_with(MainPlayButton::Play)),
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn setup(mut c: Commands)
{
    c.react()
        .on(resource_mutation::<LobbyDisplay>(), set_play_button_state);
}

//-------------------------------------------------------------------------------------------------------------------

fn make_play_button(
    ui: &mut UiBuilder<MainUi>,
    button: &Widget,
    default_button_overlay: &Widget,
    selected_button_overlay: &Widget,
) -> [Widget; 3]
{
    // get style
    let style = ui.style::<PlayButtonStyle>();

    // add default button image
    let default_button = make_overlay(ui.tree(), &default_button_overlay, "play_default", false);
    let default_image = ImageElementBundle::new(
        &default_button,
        ImageParams::center()
            .with_depth(50.)
            .with_width(Some(100.))
            .with_height(Some(100.))
            .with_color(style.default_img_color_play),
        ui.asset_server.load(style.default_img.0),
        style.default_img.1,
    );
    ui.commands().spawn(default_image);

    // add selected button image
    let selected_button = make_overlay(ui.tree(), &selected_button_overlay, "play_selected", false);
    let selected_image = ImageElementBundle::new(
        &selected_button,
        ImageParams::center()
            .with_depth(50.)
            .with_width(Some(100.))
            .with_height(Some(100.))
            .with_color(style.pressed_img_color_play),
        ui.asset_server.load(style.pressed_img.0),
        style.pressed_img.1,
    );
    ui.commands().spawn(selected_image);

    // add button text
    let text = make_overlay(ui.tree(), button, "play_text", false);
    spawn_basic_text(
        ui,
        text.clone(),
        TextParams::center().with_depth(100.).with_height(Some(40.)),
        "PLAY",
    );

    [text, default_button, selected_button]
}

//-------------------------------------------------------------------------------------------------------------------

fn make_in_lobby_button(
    ui: &mut UiBuilder<MainUi>,
    button: &Widget,
    default_button_overlay: &Widget,
    selected_button_overlay: &Widget,
) -> [Widget; 3]
{
    // get style
    let style = ui.style::<PlayButtonStyle>();

    // add default button image
    let default_button = make_overlay(ui.tree(), &default_button_overlay, "inlobby_default", false);
    let default_image = ImageElementBundle::new(
        &default_button,
        ImageParams::center()
            .with_depth(50.)
            .with_width(Some(100.))
            .with_height(Some(100.))
            .with_color(style.default_img_color_inlobby),
        ui.asset_server.load(style.default_img.0),
        style.default_img.1,
    );
    ui.commands().spawn(default_image);

    // add selected button image
    let selected_button = make_overlay(ui.tree(), &selected_button_overlay, "inlobby_selected", false);
    let selected_image = ImageElementBundle::new(
        &selected_button,
        ImageParams::center()
            .with_depth(50.)
            .with_width(Some(100.))
            .with_height(Some(100.))
            .with_color(style.pressed_img_color_inlobby),
        ui.asset_server.load(style.pressed_img.0),
        style.pressed_img.1,
    );
    ui.commands().spawn(selected_image);

    // add button text
    let text = make_overlay(ui.tree(), button, "inlobby_text", false);
    spawn_basic_text(
        ui,
        text.clone(),
        TextParams::center().with_depth(100.).with_height(Some(40.)),
        "IN LOBBY",
    );

    [text, default_button, selected_button]
}

//-------------------------------------------------------------------------------------------------------------------

/// Tag component on the main play button. Should only be updated with `CallbackWith<Toggle, [button value]>`.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum MainPlayButton
{
    Play,
    InLobby,
}

//-------------------------------------------------------------------------------------------------------------------

/// `CallbackWith<Toggle, T>` for new toggle value T.
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Toggle;

//-------------------------------------------------------------------------------------------------------------------

/// If `Selected` is added to a main menu button, deselect the play button if selected.
pub(crate) fn deselect_main_play_button_for_menu_button(
    mut commands: Commands,
    selected_main_menu_button: Query<(), (With<MainMenuButton>, Added<Selected>)>,
    selected_main_play_button: Query<
        &bevy_kot::prelude::Callback<Deselect>,
        (With<MainPlayButton>, With<Selected>),
    >,
)
{
    if selected_main_menu_button.is_empty() {
        return;
    }
    let Ok(deselect_callback) = selected_main_play_button.get_single() else {
        return;
    };
    commands.add(deselect_callback.clone());
}

//-------------------------------------------------------------------------------------------------------------------

/// If `Selected` is added to a main play button, deselect a main menu button if one is selected.
pub(crate) fn deselect_main_menu_button_for_play_button(
    mut commands: Commands,
    selected_main_play_button: Query<(), (With<MainPlayButton>, Added<Selected>)>,
    selected_main_menu_button: Query<
        &bevy_kot::prelude::Callback<Deselect>,
        (With<MainMenuButton>, With<Selected>),
    >,
)
{
    if selected_main_play_button.is_empty() {
        return;
    }
    let Ok(deselect_callback) = selected_main_menu_button.get_single() else {
        return;
    };
    commands.add(deselect_callback.clone());
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_button(ui: &mut UiBuilder<MainUi>, button: &Widget, area_overlay: &Widget) -> Entity
{
    // set text style
    ui.add_style(ui.style::<PlayButtonStyle>().text.clone());

    // prepare overlays for controlling visibility
    let default_button_overlay = make_overlay(ui.tree(), button, "default", true);
    let selected_button_overlay = make_overlay(ui.tree(), button, "selected", false);

    // prepare buttons
    let play_pack = ui.div(|ui| make_play_button(ui, button, &default_button_overlay, &selected_button_overlay));
    let inlobby_pack =
        ui.div(|ui| make_in_lobby_button(ui, button, &default_button_overlay, &selected_button_overlay));

    // build the button
    let despawner = ui.despawner.clone();
    let mut entity_commands = ui.commands().spawn_empty();
    let button_entity = entity_commands.id();

    InteractiveElementBuilder::new()
        .with_default_widget(default_button_overlay)
        .with_selected_widget(selected_button_overlay)
        .select_on_click()
        .on_select(prep_fncall((MainUi, [area_overlay.clone()], []), toggle_ui_visibility))
        .on_deselect(prep_fncall((MainUi, [], [area_overlay.clone()]), toggle_ui_visibility))
        .build::<MouseLButtonMain>(&despawner, &mut entity_commands, button.clone())
        .unwrap();

    // toggle play callback
    let toggle_play_callback =
        CallbackWith::<Toggle, MainPlayButton>::new(move |world: &mut World, button: MainPlayButton| {
            // update play button
            if !try_set_component(world, button_entity, button) {
                return;
            }

            // toggle the button's appearance
            match button {
                MainPlayButton::Play => {
                    syscall(
                        world,
                        (MainUi, play_pack.clone(), inlobby_pack.clone()),
                        toggle_ui_visibility,
                    );
                }
                MainPlayButton::InLobby => {
                    syscall(
                        world,
                        (MainUi, inlobby_pack.clone(), play_pack.clone()),
                        toggle_ui_visibility,
                    );
                }
            };
        });
    entity_commands.insert(toggle_play_callback);

    // initialize button value
    ui.commands().add(move |world: &mut World| {
        let _ = try_callback_with::<Toggle, MainPlayButton>(world, button_entity, MainPlayButton::Play);
    });

    button_entity
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiPlayButtonPlugin;

impl Plugin for UiPlayButtonPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, setup).add_systems(
            PreUpdate,
            (
                deselect_main_play_button_for_menu_button,
                deselect_main_menu_button_for_play_button,
                apply_deferred,
            )
                .chain(),
        );
    }
}

//-------------------------------------------------------------------------------------------------------------------
