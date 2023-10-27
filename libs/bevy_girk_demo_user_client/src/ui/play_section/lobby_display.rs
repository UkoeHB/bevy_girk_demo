//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::{*, syscall};
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn leave_current_lobby(
    client : Res<HostUserClient>,
    lobby  : Res<ReactRes<LobbyDisplay>>,
){
    let Some(lobby_data) = lobby.get()
    else { tracing::error!("tried to leave lobby but we aren't in a lobby"); return; };

    if let Err(err) = client.send(UserToHostMsg::LeaveLobby{ id: lobby_data.id })
    { tracing::warn!(?err, lobby_data.id, "failed sending leave lobby message to host server"); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn start_current_lobby(
    client : Res<HostUserClient>,
    lobby  : Res<ReactRes<LobbyDisplay>>,
){
    let Some(lobby_data) = lobby.get()
    else { tracing::error!("tried to start lobby but we aren't in a lobby"); return; };

    if let Err(err) = client.send(UserToHostMsg::LaunchLobbyGame{ id: lobby_data.id })
    { tracing::warn!(?err, lobby_data.id, "failed sending launch lobby message to host server"); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_title(
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
                "Current Lobby"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_box(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_leave_lobby_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    button       : &Widget,
){
    // button ui
    let button_overlay = make_overlay(ui, button, "leave_lobby", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Leave",
            |world, _| syscall(world, (), leave_current_lobby)
        );

    // block button and grey-out text when there is no lobby
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            {
                let enable = world.resource::<ReactRes<LobbyDisplay>>().is_set();
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
    rcommands.trigger_resource_mutation::<LobbyDisplay>();  //initialize
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_start_game_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    button       : &Widget,
){
    // button ui
    let button_overlay = make_overlay(ui, button, "launch_lobby", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Start",
            |world, _| syscall(world, (), start_current_lobby)
        );

    // block button and grey-out text when we don't own the lobby
    let block_overlay = make_overlay(ui, &button_overlay, "", true);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            {
                // only enable the button if we own the lobby
                let enable = match world.resource::<ReactRes<LobbyDisplay>>().get()
                {
                    Some(data) => data.owner_id == world.resource::<HostUserClient>().id(),
                    None       => false,
                };
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
    rcommands.trigger_resource_mutation::<LobbyDisplay>();  //initialize
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_buttons(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // overlay
    let buttons_overlay = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 10. },
                relative_2: Vec2{ x: 90., y: 90. },
                ..Default::default()
            }
        ).unwrap();

    // buttons: widget grid
    let lobby_button_widgets = GridSegment::new()
        .with_cells(vec![
            GridCell::named(Vec2::splat(10.0), "leave_lobby"),
            GridCell::named(Vec2::splat(10.0), "start_game")
        ])
        .add_gaps(3.0)
        .build_in(ui, &buttons_overlay, GridOrientation::Horizontal)
        .unwrap();

    // prepare each of the buttons
    add_leave_lobby_button(rcommands, asset_server, ui, &lobby_button_widgets[0]);
    add_start_game_button(rcommands, asset_server, ui, &lobby_button_widgets[1]);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_display(
    rcommands    : &mut ReactCommands,
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
    add_lobby_display_title(rcommands, asset_server, ui, &lobby_display_title);

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
    add_lobby_display_box(rcommands, asset_server, ui, &lobby_display_box);

    // leave/launch buttons
    let lobby_leave_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 65. },
                relative_2: Vec2{ x: 100., y: 80. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_buttons(rcommands, asset_server, ui, &lobby_leave_button);
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyDisplayPlugin(app: &mut App)
{
    app
        ;
}

//-------------------------------------------------------------------------------------------------------------------