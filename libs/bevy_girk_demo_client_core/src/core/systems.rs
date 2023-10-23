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

/// Helper function-system for accessing the client core mode.
pub fn get_current_client_core_mode(current_client_core_mode: Res<State<ClientCoreMode>>) -> ClientCoreMode
{
    **current_client_core_mode
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
