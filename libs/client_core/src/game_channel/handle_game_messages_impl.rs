use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_client_fw::*;
use game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Use current game state to update client state.
fn update_client_state(
    In(game_state): In<GameState>,
    client_init_state: Res<State<ClientInitState>>,
    current_client_state: Res<State<ClientState>>,
    mut next_client_state: ResMut<NextState<ClientState>>,
)
{
    // do not update game state if we are in the process of initializing the client
    if *client_init_state != ClientInitState::Done {
        return;
    }

    // update game state
    let new_client_state = match game_state {
        GameState::Init => ClientState::Init,
        GameState::Prep => ClientState::Prep,
        GameState::Play => ClientState::Play,
        GameState::GameOver => ClientState::GameOver,
    };

    if new_client_state == **current_client_state {
        return;
    }
    next_client_state.set(new_client_state);
    tracing::info!(?new_client_state, "new client state");
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle current game state.
pub(crate) fn handle_game_state(In(current_game_state): In<GameState>, world: &mut World)
{
    world.syscall(current_game_state, update_client_state);
    world.syscall((), apply_state_transitions);
}

//-------------------------------------------------------------------------------------------------------------------
