use bevy::prelude::*;
use bevy_girk_utils::new_channel;
use game_core::PlayerInput;
use wiring_game_instance::ClientInitializer;

//-------------------------------------------------------------------------------------------------------------------

/// Prepare the click client skin on a client app.
pub fn setup_client_game(world: &mut World, initializer: ClientInitializer)
{
    let (player_input_sender, player_input_receiver) = new_channel::<PlayerInput>();
    world.insert_resource(initializer);
    world.insert_resource(player_input_receiver);
    world.insert_resource(player_input_sender);
}

//-------------------------------------------------------------------------------------------------------------------
