use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_kot_ecs::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn player_syscall<A, S, Marker>(world: &mut World, id: ClientId, req: ClientRequest, arg: A, sys: S)
where
    A: Send + Sync + 'static,
    S: IntoSystem<(Entity, A), (), Marker> + Send + Sync + 'static,
{
    match world.resource::<PlayerMap>().client_to_entity(id) {
        Ok(player_entity) => syscall(world, (player_entity, arg), sys),
        Err(err) => {
            tracing::trace!(?id, ?err, "player syscall failed, client is not player");
            syscall(world, (id, req, RejectionReason::Invalid), notify_request_rejected);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_request_init(world: &mut World, id: ClientId, req: ClientRequest)
{
    match req {
        ClientRequest::GetGameMode => syscall(world, id, handle_game_mode_request),
        ClientRequest::PlayerInput(_) => {
            syscall(world, (id, req, RejectionReason::ModeMismatch), notify_request_rejected)
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_request_prep(world: &mut World, id: ClientId, req: ClientRequest)
{
    match req {
        ClientRequest::GetGameMode => syscall(world, id, handle_game_mode_request),
        ClientRequest::PlayerInput(_) => {
            syscall(world, (id, req, RejectionReason::ModeMismatch), notify_request_rejected)
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_request_play(world: &mut World, id: ClientId, req: ClientRequest)
{
    match req {
        ClientRequest::GetGameMode => syscall(world, id, handle_game_mode_request),
        ClientRequest::PlayerInput(i) => player_syscall(world, id, req, i, handle_player_input),
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_request_gameover(world: &mut World, id: ClientId, req: ClientRequest)
{
    match req {
        ClientRequest::GetGameMode => syscall(world, id, handle_game_mode_request),
        ClientRequest::PlayerInput(_) => {
            syscall(world, (id, req, RejectionReason::ModeMismatch), notify_request_rejected)
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle a message sent to the game from a client.
///
/// Note: this function is meant to be injected to a [`ClientMessageHandler`].
pub(crate) fn handle_client_request(
    world: &mut World,
    client_id: ClientId,
    client_packet: &ClientPacket,
) -> Result<(), Option<ClientFwRequest>>
{
    let request = deserialize_client_request(client_id, client_packet)?;

    match syscall(world, (), get_current_game_mode) {
        GameMode::Init => handle_client_request_init(world, client_id, request),
        GameMode::Prep => handle_client_request_prep(world, client_id, request),
        GameMode::Play => handle_client_request_play(world, client_id, request),
        GameMode::GameOver => handle_client_request_gameover(world, client_id, request),
    }

    Ok(())
}

//-------------------------------------------------------------------------------------------------------------------
