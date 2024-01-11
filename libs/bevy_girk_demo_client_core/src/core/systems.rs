//! Miscellaneous systems.

//local shortcuts
use crate::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_client_fw::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Helper function-system for accessing the client mode.
pub fn get_current_client_mode(current_client_mode: Res<State<ClientMode>>) -> ClientMode
{
    **current_client_mode
}

//-------------------------------------------------------------------------------------------------------------------

/// Request the current game mode.
pub(crate) fn request_game_mode(buffer: Res<ClientRequestBuffer>)
{
    buffer.request(ClientRequest::GetGameMode);
}

//-------------------------------------------------------------------------------------------------------------------

pub fn send_client_request(In(msg): In<ClientRequest>, buffer: Res<ClientRequestBuffer>)
{
    buffer.request(msg);
}

//-------------------------------------------------------------------------------------------------------------------
