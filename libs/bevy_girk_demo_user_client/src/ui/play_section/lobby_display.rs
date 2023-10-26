//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::ecs::{*, syscall};
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_title(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    let display_title_text = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //center
                relative_1: Vec2{ x: 20., y: 35. },
                relative_2: Vec2{ x: 80., y: 65. },
                ..Default::default()
            }
        ).unwrap();

    let display_title_text_style = TextStyle {
            font      : asset_server.load(TEMP_FONT),
            font_size : 45.0,
            color     : TEMP_FONT_COLOR,
        };

    commands.spawn(
            TextElementBundle::new(
                display_title_text,
                TextParams::center()
                    .with_style(&display_title_text_style)
                    .with_height(Some(100.)),
                "Current Lobby"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_box(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_leave_button(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_start_game_button(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_new_lobby_button(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_display(
    commands     : &mut Commands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // title
    let lobby_display_title = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 20. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_title(commands, asset_server, ui, &lobby_display_title);

    // display box
    let lobby_display_box = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 20. },
                relative_2: Vec2{ x: 100., y: 65. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_box(commands, asset_server, ui, &lobby_display_box);

    // leave button
    let lobby_leave_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 65. },
                relative_2: Vec2{ x: 50., y: 80. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_leave_button(commands, asset_server, ui, &lobby_leave_button);

    // start game button
    let start_game_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 50., y: 65. },
                relative_2: Vec2{ x: 100., y: 80. },
                ..Default::default()
            }
        ).unwrap();
    add_start_game_button(commands, asset_server, ui, &start_game_button);

    // new lobby button
    let new_lobby_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 80. },
                relative_2: Vec2{ x: 100., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_new_lobby_button(commands, asset_server, ui, &new_lobby_button);
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyDisplayPlugin(app: &mut App)
{
    app
        ;
}

//-------------------------------------------------------------------------------------------------------------------
