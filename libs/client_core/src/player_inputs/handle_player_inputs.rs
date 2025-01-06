use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_kot_utils::*;
use game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn send_client_request(In(msg): In<ClientRequest>, mut sender: ClientRequestSender)
{
    sender.request(msg);
}

//-------------------------------------------------------------------------------------------------------------------

fn process_player_inputs(world: &mut World, state: ClientState, handler: impl Fn(&mut World, PlayerInput, ClientState))
{
    let Some(player_inputs) = world.remove_resource::<Receiver<PlayerInput>>() else {
        return;
    };

    while let Some(input) = player_inputs.try_recv() {
        if world.resource::<ClientContext>().client_type() != ClientType::Player {
            tracing::warn!(?input, "ignoring input sent by a non-player client");
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
        ClientState::Play => world.syscall(ClientRequest::PlayerInput(player_input), send_client_request),
        _ => {
            tracing::warn!(?input, "ignoring invalid input sent during {state:?}");
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientMode::GameOver.
pub(crate) fn handle_player_inputs(world: &mut World)
{
    let Some(state) = world.get_resource::<State<ClientState>>() else { return; };

    process_player_inputs(world, state, handle_input);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn clear_player_inputs(world: &mut World)
{
    process_player_inputs(world, ClientState::Init, |_, _, _| {});
}

//-------------------------------------------------------------------------------------------------------------------
