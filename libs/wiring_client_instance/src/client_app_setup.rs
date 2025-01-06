use bevy::prelude::*;
use bevy_kot_utils::*;
use bevy_replicon::prelude::*;
use client_core::*;
use client_skin::*;

//-------------------------------------------------------------------------------------------------------------------

/// Prepare the click client skin on a client app.
pub fn setup_client_game(world: &mut World, initializer: ClientInitializer)
{
    let (player_input_sender, player_input_receiver) = new_channel::<PlayerInput>();
    world
        .insert_resource(initializer)
        .insert_resource(player_input_receiver)
        .insert_resource(player_input_sender);
}

//-------------------------------------------------------------------------------------------------------------------
