//local shortcuts
use crate::*;
use bevy_girk_client_fw::*;

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Initialize the game output handler.
pub(crate) fn setup_game_output_handler(mut commands: Commands)
{
    commands.insert_resource::<GameMessageHandler>(GameMessageHandler::new(try_handle_game_core_output));
}

//-------------------------------------------------------------------------------------------------------------------
