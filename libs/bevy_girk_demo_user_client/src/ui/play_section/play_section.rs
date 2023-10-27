//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::ecs::*;
use bevy_kot::ui::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_play_overlay(
    rcommands    : &mut ReactCommands,
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
    add_lobby_display(rcommands, asset_server, ui, &lobby_display_area);

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
    add_lobby_list(rcommands, asset_server, ui, &lobby_list_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_section(
    rcommands    : &mut ReactCommands,
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
    let _ = add_play_button(rcommands, ui, &play_button_area, &play_overlay, asset_server);
    add_play_overlay(rcommands, ui, &play_overlay, asset_server);
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
