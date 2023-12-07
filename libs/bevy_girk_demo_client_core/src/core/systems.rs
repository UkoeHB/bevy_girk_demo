//! Miscellaneous systems.

//local shortcuts
use crate::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_utils::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Helper function-system for accessing the client mode.
pub fn get_current_client_mode(current_client_mode: Res<State<ClientMode>>) -> ClientMode
{
    **current_client_mode
}

//-------------------------------------------------------------------------------------------------------------------

/// Request the current game mode.
pub(crate) fn request_game_mode(mut client_message_buffer: ResMut<ClientMessageBuffer>)
{
    client_message_buffer.add_core_msg(&GameRequest::GameModeRequest, SendUnordered);
}

//-------------------------------------------------------------------------------------------------------------------

pub fn send_game_request(In(msg): In<GameRequest>, mut client_message_buffer: ResMut<ClientMessageBuffer>)
{
    client_message_buffer.add_core_msg(&msg, SendOrdered);
}

//-------------------------------------------------------------------------------------------------------------------
