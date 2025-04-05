use bevy::prelude::*;
use bevy_cobweb::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn player_syscall<A, S, Marker>(world: &mut World, id: ClientId, req: ClientRequest, arg: A, sys: S)
where
    A: Send + Sync + 'static,
    S: IntoSystem<In<(Entity, A)>, (), Marker> + Send + Sync + 'static,
{
    match world.resource::<PlayerMap>().client_to_entity(id) {
        Ok(player_entity) => world.syscall((player_entity, arg), sys),
        Err(err) => {
            tracing::trace!(?id, ?err, "player syscall failed, client is not player");
            world.syscall((id, req, RejectionReason::Invalid), notify_request_rejected);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle a request sent to the game from a client.
///
/// Note: this function is meant to be injected to a [`ClientMessageHandler`].
pub(crate) fn handle_client_request(world: &mut World, id: ClientId, req: ClientRequest)
{
    let state = world.syscall((), get_game_state);
    let reject = |world: &mut World| {
        world.syscall((id, req, RejectionReason::ModeMismatch), notify_request_rejected);
    };

    match req {
        ClientRequest::GetGameState => world.syscall(id, handle_game_state_request),
        ClientRequest::PlayerInput(i) => match state {
            GameState::Play => player_syscall(world, id, req, i, handle_player_input),
            _ => reject(world),
        },
    }
}

//-------------------------------------------------------------------------------------------------------------------
