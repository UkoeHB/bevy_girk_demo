//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn set_play_button_state(
    mut commands : Commands,
    display      : Res<ReactRes<LobbyDisplay>>,
    callback     : Query<&CallbackWith<Toggle, MainPlayButton>>,
){
    let cb = callback.single();

    match display.is_set()
    {
        true  => commands.add(cb.call_with(MainPlayButton::InLobby)),
        false => commands.add(cb.call_with(MainPlayButton::Play)),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup(mut rcommands: ReactCommands)
{
    rcommands.on_resource_mutation::<ReactRes<LobbyDisplay>>(
            |world: &mut World| { syscall(world, (), set_play_button_state); }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_play_button(
    ui                      : &mut UiBuilder<MainUI>,
    button                  : &Widget,
    default_button_overlay  : &Widget,
    selected_button_overlay : &Widget,
) -> [Widget; 3]
{
    // add default button image
    let default_button = make_overlay(ui.tree(), &default_button_overlay, "play_default", false);
    let default_image = ImageElementBundle::new(
            &default_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(Color::BISQUE),  //tint the default button (todo: it's ugly)
                ui.asset_server.load(MENU_BAR_BUTTON.0),
                MENU_BAR_BUTTON.1
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
                .with_color(Color::OLIVE),  //tint the default button (todo: it's ugly)
                ui.asset_server.load(MENU_BAR_BUTTON.0),
                MENU_BAR_BUTTON.1
        );
    ui.commands().spawn(selected_image);

    // set text style
    ui.add(basic_text_default_light_style());

    // add button text
    let text = make_overlay(ui.tree(), button, "play_text", false);
    spawn_basic_text(
            ui,
            text.clone(),
            TextParams::center()
                .with_depth(100.)
                .with_height(Some(40.)),
            "PLAY"
        );

    [text, default_button, selected_button]
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_in_lobby_button(
    ui                      : &mut UiBuilder<MainUI>,
    button                  : &Widget,
    default_button_overlay  : &Widget,
    selected_button_overlay : &Widget,
) -> [Widget; 3]
{
    // add default button image
    let default_button = make_overlay(ui.tree(), &default_button_overlay, "inlobby_default", false);
    let default_image = ImageElementBundle::new(
            &default_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(Color::YELLOW),  //use tint to differentiate (todo: it's ugly)
            ui.asset_server.load(MENU_BAR_BUTTON.0),
            MENU_BAR_BUTTON.1
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
                .with_color(Color::OLIVE),  //tint the default button (todo: it's ugly)
            ui.asset_server.load(MENU_BAR_BUTTON.0),
            MENU_BAR_BUTTON.1
        );
    ui.commands().spawn(selected_image);

    // set text style
    ui.add(basic_text_default_light_style());

    // add button text
    let text = make_overlay(ui.tree(), button, "inlobby_text", false);
    spawn_basic_text(
            ui,
            text.clone(),
            TextParams::center()
                .with_depth(100.)
                .with_height(Some(40.)),
            "IN LOBBY"
        );

    [text, default_button, selected_button]
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Tag component on the main play button. Should only be updated with `CallbackWith<Toggle, [button value]>`.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum MainPlayButton
{
    Play,
    #[allow(dead_code)]
    InLobby,
}

//-------------------------------------------------------------------------------------------------------------------

/// `CallbackWith<Toggle, T>` for new toggle value T.
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Toggle;

//-------------------------------------------------------------------------------------------------------------------

/// If `Selected` is added to a main menu button, deselect the play button if selected.
pub(crate) fn deselect_main_play_button_for_menu_button(
    mut commands              : Commands,
    selected_main_menu_button : Query<(), (With<MainMenuButton>, Added<Selected>)>,
    selected_main_play_button : Query<&Callback<Deselect>, (With<MainPlayButton>, With<Selected>)>,
){
    if selected_main_menu_button.is_empty() { return; }
    let Ok(deselect_callback) = selected_main_play_button.get_single() else { return; };
    commands.add(deselect_callback.clone());
}

//-------------------------------------------------------------------------------------------------------------------

/// If `Selected` is added to a main play button, deselect a main menu button if one is selected.
pub(crate) fn deselect_main_menu_button_for_play_button(
    mut commands              : Commands,
    selected_main_play_button : Query<(), (With<MainPlayButton>, Added<Selected>)>,
    selected_main_menu_button : Query<&Callback<Deselect>, (With<MainMenuButton>, With<Selected>)>,
){
    if selected_main_play_button.is_empty() { return; }
    let Ok(deselect_callback) = selected_main_menu_button.get_single() else { return; };
    commands.add(deselect_callback.clone());
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_button(ui: &mut UiBuilder<MainUI>, button: &Widget, area_overlay: &Widget) -> Entity
{
    // prepare overlays for controlling visibility
    let default_button_overlay = make_overlay(ui.tree(), button, "default", true);
    let selected_button_overlay = make_overlay(ui.tree(), button, "selected", false);

    // prepare buttons
    let play_pack = ui.div(|ui| make_play_button(ui, button, &default_button_overlay, &selected_button_overlay));
    let inlobby_pack = ui.div(|ui| make_in_lobby_button(ui, button, &default_button_overlay, &selected_button_overlay));

    // select/deselect callbacks
    let mut entity_commands = ui.commands().spawn_empty();
    let button_entity = entity_commands.id();
    let area_overlay_clone = area_overlay.clone();
    let select_callback =
        move |world: &mut World|
        {
            syscall(world, (MainUI, [area_overlay_clone.clone()], []), toggle_ui_visibility);
        };
    let area_overlay_clone = area_overlay.clone();
    let deselect_callback =
        move |world: &mut World|
        {
            syscall(world, (MainUI, [], [area_overlay_clone.clone()]), toggle_ui_visibility);
        };

    // build the button
    InteractiveElementBuilder::new()
        .with_default_widget(default_button_overlay)
        .with_selected_widget(selected_button_overlay)
        .select_on_click()
        .select_callback(select_callback)
        .deselect_callback(deselect_callback)
        .build::<MouseLButtonMain>(&mut entity_commands, button.clone())
        .unwrap();

    // toggle play callback
    let toggle_play_callback = CallbackWith::<Toggle, MainPlayButton>::new(
            move |world: &mut World, button: MainPlayButton|
            {
                // update play button
                if !try_set_component(world, button_entity, button) { return; }

                // toggle the button's appearance
                match button
                {
                    MainPlayButton::Play =>
                    {
                        syscall(world, (MainUI, play_pack.clone(), inlobby_pack.clone()), toggle_ui_visibility);
                    }
                    MainPlayButton::InLobby =>
                    {
                        syscall(world, (MainUI, inlobby_pack.clone(), play_pack.clone()), toggle_ui_visibility);
                    }
                };
            }
        );
    entity_commands.insert(toggle_play_callback);

    // initialize button value
    ui.commands().add(
            move |world: &mut World|
            { let _ = try_callback_with::<Toggle, MainPlayButton>(world, button_entity, MainPlayButton::Play); }
        );

    button_entity
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiPlayButtonPlugin(app: &mut App)
{
    app.add_systems(Startup, setup)
        .add_systems(PreUpdate,
            (
                deselect_main_play_button_for_menu_button,
                deselect_main_menu_button_for_play_button,
                apply_deferred,
            ).chain()
        );
}

//-------------------------------------------------------------------------------------------------------------------
