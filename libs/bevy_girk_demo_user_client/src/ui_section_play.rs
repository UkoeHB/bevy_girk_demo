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

fn make_play_button(
    commands                : &mut Commands,
    ui                      : &mut UiTree,
    button                  : &Widget,
    default_button_overlay  : &Widget,
    selected_button_overlay : &Widget,
    asset_server            : &AssetServer,
) -> [Widget; 3]
{
    // add default button image
    let default_button = make_overlay(ui, &default_button_overlay, "play_default", false);
    commands.spawn(
        ImageElementBundle::new(
                &default_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::BISQUE),  //tint the default button (todo: it's ugly)
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add selected button image
    let selected_button = make_overlay(ui, &selected_button_overlay, "play_selected", false);
    commands.spawn(
        ImageElementBundle::new(
                &selected_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::OLIVE),  //tint the default button (todo: it's ugly)
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add button text
    let menu_bar_text_style = TextStyle {
            font      : asset_server.load(MENU_BAR_BUTTON_FONT),
            font_size : 40.0,
            color     : MENU_BAR_BUTTON_FONT_COLOR,
        };

    let play_button_text = make_overlay(ui, button, "play_text", false);
    commands.spawn(
        TextElementBundle::new(
                &play_button_text,
                TextParams::center()
                    .with_style(&menu_bar_text_style)
                    .with_depth(100.)
                    .with_height(Some(40.)),
                "PLAY"
            )
    );

    [play_button_text, default_button, selected_button]
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_in_lobby_button(
    commands                : &mut Commands,
    ui                      : &mut UiTree,
    button                  : &Widget,
    default_button_overlay  : &Widget,
    selected_button_overlay : &Widget,
    asset_server            : &AssetServer,
) -> [Widget; 3]
{
    // add default button image
    let default_button = make_overlay(ui, &default_button_overlay, "inlobby_default", false);
    commands.spawn(
        ImageElementBundle::new(
                &default_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::YELLOW),  //use tint to differentiate (todo: it's ugly)
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add selected button image
    let selected_button = make_overlay(ui, &selected_button_overlay, "inlobby_selected", false);
    commands.spawn(
        ImageElementBundle::new(
                &selected_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::OLIVE),  //tint the default button (todo: it's ugly)
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add button text
    let menu_bar_text_style = TextStyle {
            font      : asset_server.load(MENU_BAR_BUTTON_FONT),
            font_size : 40.0,
            color     : MENU_BAR_BUTTON_FONT_COLOR,
        };

    let inlobby_button_text = make_overlay(ui, button, "inlobby_text", false);
    commands.spawn(
        TextElementBundle::new(
                &inlobby_button_text,
                TextParams::center()
                    .with_style(&menu_bar_text_style)
                    .with_depth(100.)
                    .with_height(Some(40.)),
                "IN LOBBY"
            )
    );

    [inlobby_button_text, default_button, selected_button]
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_play_button(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    button       : &Widget,
    area_overlay : &Widget,
    asset_server : &AssetServer,
) -> Entity
{
    // prepare overlays for controlling visibility
    let default_button_overlay = make_overlay(ui, button, "default", true);
    let selected_button_overlay = make_overlay(ui, button, "selected", false);

    // prepare buttons
    let play_pack = make_play_button(
            commands,
            ui,
            button,
            &default_button_overlay,
            &selected_button_overlay,
            asset_server
        );
    let inlobby_pack = make_in_lobby_button(
            commands,
            ui,
            button,
            &default_button_overlay,
            &selected_button_overlay,
            asset_server
        );

    // select/deselect callbacks
    let mut entity_commands = commands.spawn_empty();
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

    // build interactivity into the widget
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
    commands.add(
            move |world: &mut World|
            { let _ = try_callback_with::<Toggle, MainPlayButton>(world, button_entity, MainPlayButton::Play); }
        );

    button_entity
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_experimental_button(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    button       : &Widget,
    asset_server : &AssetServer,
) -> (Widget, Widget, Widget)
{
    let button_overlay = make_overlay(ui, button, "experiment", true);

    // add default button image
    let default_button = make_overlay(ui, &button_overlay, "default", true);
    commands.spawn(
        ImageElementBundle::new(
                &default_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::WHITE),
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add pressed button image
    let pressed_button = make_overlay(ui, &button_overlay, "pressed", false);
    commands.spawn(
        ImageElementBundle::new(
                &pressed_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::DARK_GRAY),
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add hovered button image
    let hovered_button = make_overlay(ui, &button_overlay, "hovered", true);
    commands.spawn(
        ImageElementBundle::new(
                &hovered_button,
                ImageParams::center()
                    .with_depth(50.)
                    .with_width(Some(100.))
                    .with_height(Some(100.))
                    .with_color(Color::GRAY),
                asset_server.load(MENU_BAR_BUTTON),
                Vec2::new(250.0, 142.0)
            )
    );

    // add button text
    let text_style = TextStyle {
            font      : asset_server.load(MENU_BAR_BUTTON_FONT),
            font_size : 40.0,
            color     : MENU_BAR_BUTTON_FONT_COLOR,
        };

    commands.spawn(
        TextElementBundle::new(
                &button_overlay,
                TextParams::center()
                    .with_style(&text_style)
                    .with_depth(100.)
                    .with_width(Some(80.)),
                "CLICK ME"
            )
    );

    (default_button, pressed_button, hovered_button)
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_experimental_button(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    button       : &Widget,
    asset_server : &AssetServer,
){
    // prepare button widgets
    let (default_button, pressed_button, hovered_button) = make_experimental_button(
            commands,
            ui,
            button,
            asset_server
        );

    // prepare press home zone
    // - the zone extends beyond the underlying button area as a buffer for click releases
    let press_home_zone = Widget::create(
            ui,
            button.end("press_home_zone"),
            RelativeLayout{
                relative_1: Vec2{ x: -20., y: -45. },
                relative_2: Vec2{ x: 120., y: 145. },
                ..Default::default()
            }
        ).unwrap();

    // build the click-and-release behavior with the provided widgets
    let mut entity_commands = commands.spawn_empty();

    InteractiveElementBuilder::new()
        .with_default_widget(default_button)
        .with_pressed_widget(pressed_button)
        .with_hovered_widget(hovered_button)
        .with_press_home_zone(press_home_zone)
        .press_on_click()
        .unpress_on_unclick_home_and_abort_on_unclick_away()
        .abort_press_if_obstructed()
        .no_hover_on_pressed()
        .build::<MouseLButtonMain>(&mut entity_commands, button.clone())
        .unwrap();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_play_overlay(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    overlay      : &Widget,
    asset_server : &AssetServer
){
    // experimental clickable button
    let experimental_button = Widget::create(
            ui,
            overlay.end(""),
            RelativeLayout{  //center top
                relative_1: Vec2{ x: 40., y: 20. },
                relative_2: Vec2{ x: 60., y: 40. },
                ..Default::default()
            }
        ).unwrap();
    add_experimental_button(commands, ui, &experimental_button, asset_server);

    // overlay
    let play_overlay_text = Widget::create(
            ui,
            overlay.end(""),
            RelativeLayout{  //center bottom
                relative_1: Vec2{ x: 40., y: 50. },
                relative_2: Vec2{ x: 60., y: 70. },
                ..Default::default()
            }
        ).unwrap();

    let play_overlay_text_style = TextStyle{
            font      : asset_server.load(TEMP_FONT),
            font_size : 45.0,
            color     : TEMP_FONT_COLOR,
        };

    commands.spawn(
            TextElementBundle::new(
                play_overlay_text,
                TextParams::center().with_style(&play_overlay_text_style),
                "PLAY OVERLAY: TBD"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
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

/// Tag component on the main play button. Should only be updated with `CallbackWith<Toggle, [button value]>`.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum MainPlayButton
{
    Play,
    #[allow(dead_code)]
    InLobby,
}

pub(crate) fn add_play_section(
    commands     : &mut Commands,
    ui           : &mut UiTree,
    play_button  : Widget,
    menu_overlay : Widget,
    asset_server : &AssetServer
){
    let play_button_area = Widget::create(
            ui,
            play_button.end(""),
            RelativeLayout{  //with buffer around edge
                absolute_1: Vec2{ x: 1., y: 2.5 },
                absolute_2: Vec2{ x: -1., y: -2.5 },
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    let play_overlay = make_overlay(ui, &menu_overlay, "play_overlay", false);
    let _ = add_play_button(commands, ui, &play_button_area, &play_overlay, asset_server);
    add_play_overlay(commands, ui, &play_overlay, asset_server);
}

//-------------------------------------------------------------------------------------------------------------------

/// `CallbackWith<Toggle, T>` for new toggle value T.
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Toggle;

//-------------------------------------------------------------------------------------------------------------------
