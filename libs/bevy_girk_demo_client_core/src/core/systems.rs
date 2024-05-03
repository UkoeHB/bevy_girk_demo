//! Miscellaneous systems.

use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_demo_game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Helper function-system for accessing the client mode.
pub fn get_current_client_mode(current_client_mode: Res<State<ClientMode>>) -> ClientMode
{
    **current_client_mode
}

//-------------------------------------------------------------------------------------------------------------------

/// Request the current game mode.
pub(crate) fn request_game_mode(mut sender: ClientRequestSender)
{
    sender.request(ClientRequest::GetGameMode);
}

//-------------------------------------------------------------------------------------------------------------------

pub fn send_client_request(In(msg): In<ClientRequest>, mut sender: ClientRequestSender)
{
    sender.request(msg);
}

//-------------------------------------------------------------------------------------------------------------------
