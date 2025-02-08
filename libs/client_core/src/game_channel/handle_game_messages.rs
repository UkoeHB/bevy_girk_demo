use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_game_fw::*;
use game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn handle_request_rejected(request: ClientRequest, reason: RejectionReason)
{
    tracing::warn!("game request {request:?} rejected: {reason:?}");
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle a message sent to the client from the game.
///
/// Note: this function is meant to be injected to a [`GameMessageHandler`], where it will be invoked by the client
///       framework at the start of each tick to handle incoming game messages.
pub(crate) fn handle_game_message(world: &mut World, _tick: Tick, message: GameMsg)
{
    let _state = **world.resource::<State<ClientState>>();

    match message {
        GameMsg::RequestRejected { reason, request } => handle_request_rejected(request, reason),
        GameMsg::CurrentGameState(game_state) => world.syscall(game_state, handle_game_state),
    }
}

//-------------------------------------------------------------------------------------------------------------------
