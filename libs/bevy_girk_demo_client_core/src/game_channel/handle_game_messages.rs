//local shortcuts
use crate::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_game_fw::*;
use bevy_kot_ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_request_rejected(reason: RejectionReason, request: ClientRequest)
{
    tracing::warn!(?reason, ?request, "game request rejected");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_game_core_output_init(world: &mut World, _tick: Ticks, message: GameMsg)
{
    match message
    {
        GameMsg::RequestRejected{reason, request} => handle_request_rejected(reason, request),
        GameMsg::CurrentGameMode(mode)            => syscall(world, mode, handle_current_game_mode),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_game_core_output_prep(world: &mut World, _tick: Ticks, message: GameMsg)
{
    match message
    {
        GameMsg::RequestRejected{reason, request} => handle_request_rejected(reason, request),
        GameMsg::CurrentGameMode(mode)            => syscall(world, mode, handle_current_game_mode),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_game_core_output_play(world: &mut World, _tick: Ticks, message: GameMsg)
{
    match message
    {
        GameMsg::RequestRejected{reason, request} => handle_request_rejected(reason, request),
        GameMsg::CurrentGameMode(mode)            => syscall(world, mode, handle_current_game_mode),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_game_core_output_gameover(world: &mut World, _tick: Ticks, message: GameMsg)
{
    match message
    {
        GameMsg::RequestRejected{reason, request} => handle_request_rejected(reason, request),
        GameMsg::CurrentGameMode(mode)            => syscall(world, mode, handle_current_game_mode),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Handle a message sent to the client from the game.
///
/// Note: this function is meant to be injected to a [`GameMessageHandler`], where it will be invoked by the client
///       framework at the start of each tick to handle incoming game messages.
pub(crate) fn handle_game_message(world: &mut World, game_packet: &GamePacket) -> Result<(), Option<(Ticks, GameFwMsg)>>
{
    let (ticks, message) = deserialize_game_message(game_packet)?;

    // handle based on current client mode
    match syscall(world, (), get_current_client_mode)
    {
        ClientMode::Init     => handle_game_core_output_init(world, ticks, message),
        ClientMode::Prep     => handle_game_core_output_prep(world, ticks, message),
        ClientMode::Play     => handle_game_core_output_play(world, ticks, message),
        ClientMode::GameOver => handle_game_core_output_gameover(world, ticks, message),
    }

    Ok(())
}

//-------------------------------------------------------------------------------------------------------------------
