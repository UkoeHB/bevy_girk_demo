use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_client_fw::ClientSender;
use bevy_girk_utils::Receiver;
use game_core::*;
use wiring_game_instance::{ClientContext, ClientType};

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn send_client_request(In(request): In<ClientRequest>, mut sender: ClientSender)
{
    sender.send(request);
}

//-------------------------------------------------------------------------------------------------------------------

fn process_player_inputs(
    world: &mut World,
    state: ClientState,
    handler: impl Fn(&mut World, PlayerInput, ClientState),
)
{
    let Some(player_inputs) = world.remove_resource::<Receiver<PlayerInput>>() else {
        return;
    };

    while let Some(input) = player_inputs.try_recv() {
        if world.resource::<ClientContext>().client_type() != ClientType::Player {
            tracing::warn!("ignoring input sent by a non-player client: {input:?}");
            continue;
        }

        handler(world, input, state);
    }

    world.insert_resource(player_inputs);
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_input(world: &mut World, input: PlayerInput, state: ClientState)
{
    match state {
        ClientState::Play => world.syscall(ClientRequest::PlayerInput(input), send_client_request),
        _ => {
            tracing::warn!("ignoring invalid input sent during {state:?}: {input:?}");
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientMode::GameOver.
pub(crate) fn handle_player_inputs(world: &mut World)
{
    let Some(state) = world.get_resource::<State<ClientState>>() else {
        return;
    };

    process_player_inputs(world, **state, handle_input);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn clear_player_inputs(world: &mut World)
{
    process_player_inputs(world, ClientState::Init, |_, _, _| {});
}

//-------------------------------------------------------------------------------------------------------------------
