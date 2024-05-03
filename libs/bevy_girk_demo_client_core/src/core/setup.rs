use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_demo_game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the game output handler.
pub(crate) fn setup_game_output_handler(mut commands: Commands)
{
    commands.insert_resource(GameMessageHandler::new(handle_game_message));
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the client request buffer.
pub(crate) fn setup_client_request_buffer(mut commands: Commands)
{
    commands.insert_resource(ClientRequestType::new::<ClientRequest>());
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the client state.
pub(crate) fn setup_client_state(world: &mut World)
{
    let initializer = world
        .remove_resource::<ClientInitializer>()
        .expect("initializer missing");
    world.insert_resource(initializer.context);
}

//-------------------------------------------------------------------------------------------------------------------
