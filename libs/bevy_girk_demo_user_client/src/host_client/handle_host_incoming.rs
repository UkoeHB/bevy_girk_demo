//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_host_incoming(world: &mut World)
{
    while let Some(server_val) = world.resource::<HostUserClient>().next_val()
    {
        match server_val
        {
            HostUserServerVal::Msg(msg) =>
            {
                match msg
                {
                    HostToUserMsg::LobbyState{ lobby }          => syscall(world, lobby, handle_lobby_state_update),
                    HostToUserMsg::LobbyLeave{ id }             => syscall(world, id, handle_lobby_leave),
                    HostToUserMsg::PendingLobbyAckRequest{ id } => syscall(world, id, handle_pending_lobby_ack_request),
                    HostToUserMsg::PendingLobbyAckFail{ id }    => syscall(world, id, handle_pending_lobby_ack_fail),
                    HostToUserMsg::GameStart{ id, connect }     => syscall(world, (id, connect), handle_game_start),
                    HostToUserMsg::GameAborted{ id }            => syscall(world, id, handle_game_aborted),
                    HostToUserMsg::GameOver{ id, report }       => syscall(world, (id, report), handle_game_over),
                }
            }
            HostUserServerVal::Response(resp, request_id) =>
            {
                match resp
                {
                    HostToUserResponse::LobbySearchResult(result) =>
                    {
                        syscall(world, (request_id, result), handle_lobby_search_result);
                    }
                    HostToUserResponse::LobbyJoin{ lobby } => syscall(world, (request_id, lobby), handle_lobby_join),
                }
            }
            HostUserServerVal::Ack(request_id)          => syscall(world, request_id, handle_request_ack),
            HostUserServerVal::Reject(request_id)       => syscall(world, request_id, handle_request_rejected),
            HostUserServerVal::SendFailed(request_id)   => syscall(world, request_id, handle_send_failed),
            HostUserServerVal::ResponseLost(request_id) => syscall(world, request_id, handle_response_lost),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
