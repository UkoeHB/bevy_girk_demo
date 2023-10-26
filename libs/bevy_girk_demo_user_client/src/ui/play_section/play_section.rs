//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


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
    // lobby display area
    let lobby_display_area = Widget::create(ui, overlay.end(""),
            RelativeLayout{  //left half
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 50., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display(commands, asset_server, ui, &lobby_display_area);

    // lobby list area
    let lobby_list_area = Widget::create(
            ui,
            overlay.end(""),
            RelativeLayout{  //right half
                relative_1: Vec2{ x: 50., y: 0. },
                relative_2: Vec2{ x: 100., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list(commands, asset_server, ui, &lobby_list_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_section(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    play_button  : Widget,
    menu_overlay : Widget,
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

#[bevy_plugin]
pub(crate) fn UiPlaySectionPlugin(app: &mut App)
{
    app.add_plugins(UiPlayButtonPlugin)
        .add_plugins(UiLobbyDisplayPlugin)
        .add_plugins(UiLobbyListPlugin)
        ;
}

//-------------------------------------------------------------------------------------------------------------------
