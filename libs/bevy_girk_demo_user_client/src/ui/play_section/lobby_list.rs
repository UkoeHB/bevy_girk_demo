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

fn add_lobby_list_title(
    rcommands    : &mut ReactCommands,
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
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : MISC_FONT_COLOR,
        };

    rcommands.commands().spawn(
            TextElementBundle::new(
                display_title_text,
                TextParams::center()
                    .with_style(&display_title_text_style)
                    .with_height(Some(100.)),
                "Lobby List"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_subsection(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_new_lobby_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // overlay
    let button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 25., y: 10. },
                relative_2: Vec2{ x: 75., y: 90. },
                ..Default::default()
            }
        ).unwrap();

    // button ui
    let button_overlay = make_overlay(ui, &button_area, "new_lobby", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "New Lobby",
            |world, _| () //syscall(world, (), activate_new_lobby_window)  //TODO
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_list(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // title
    let lobby_list_title = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 20. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_title(rcommands, asset_server, ui, &lobby_list_title);

    // display box
    let lobby_list_subsection = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 20. },
                relative_2: Vec2{ x: 100., y: 65. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_subsection(rcommands, asset_server, ui, &lobby_list_subsection);

    // new lobby button
    let new_lobby_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 65. },
                relative_2: Vec2{ x: 100., y: 80. },
                ..Default::default()
            }
        ).unwrap();
    add_new_lobby_button(rcommands, asset_server, ui, &new_lobby_button);
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyListPlugin(app: &mut App)
{
    app
        ;
}

//-------------------------------------------------------------------------------------------------------------------
