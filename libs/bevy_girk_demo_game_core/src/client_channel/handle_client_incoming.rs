//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_girk_utils::*;
use bevy_kot_ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn player_syscall<A, S, Marker>(world: &mut World, req: GameRequest, id: ClientIdType, arg: A, sys: S)
where
    A: Send + Sync + 'static,
    S: IntoSystem<(Entity, A), (), Marker> + Send + Sync + 'static,
{
    match world.resource::<PlayerMap>().client_to_entity(id)
    {
        Ok(player_entity) => syscall(world, (player_entity, arg), sys),
        Err(err) =>
        {
            tracing::trace!(id, ?err, "player syscall failed, client is not player");
            syscall(world, (id, req, RejectionReason::Invalid), notify_request_rejected);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_client_input_init(world: &mut World, req: GameRequest, id: ClientIdType)
{
    match req
    {
        GameRequest::GameModeRequest => syscall(world, id, handle_game_mode_request),
        GameRequest::PlayerInput(_)  => syscall(world, (id, req, RejectionReason::ModeMismatch), notify_request_rejected),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_client_input_prep(world: &mut World, req: GameRequest, id: ClientIdType)
{
    match req
    {
        GameRequest::GameModeRequest => syscall(world, id, handle_game_mode_request),
        GameRequest::PlayerInput(_)  => syscall(world, (id, req, RejectionReason::ModeMismatch), notify_request_rejected),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_client_input_play(world: &mut World, req: GameRequest, id: ClientIdType)
{
    match req
    {
        GameRequest::GameModeRequest => syscall(world, id, handle_game_mode_request),
        GameRequest::PlayerInput(i)  => player_syscall(world, req, id, i, handle_player_input),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_client_input_gameover(world: &mut World, req: GameRequest, id: ClientIdType)
{
    match req
    {
        GameRequest::GameModeRequest => syscall(world, id, handle_game_mode_request),
        GameRequest::PlayerInput(_)  => syscall(world, (id, req, RejectionReason::ModeMismatch), notify_request_rejected),
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Handle a message sent to the game from a client.
///
/// Note: this function is meant to be injected to a [`ClientMessageHandler`].
pub(crate) fn try_handle_game_core_input(world: &mut World, serialized_message: Vec<u8>, client_id: ClientIdType) -> bool
{
    let Some(request) = deser_msg::<GameRequest>(&serialized_message[..])
    else { tracing::trace!("failed deserializing client message"); return false; };

    match syscall(world, (), get_current_game_mode)
    {
        GameMode::Init     => handle_client_input_init(world, request, client_id),
        GameMode::Prep     => handle_client_input_prep(world, request, client_id),
        GameMode::Play     => handle_client_input_play(world, request, client_id),
        GameMode::GameOver => handle_client_input_gameover(world, request, client_id),
    }

    return true;
}

//-------------------------------------------------------------------------------------------------------------------
