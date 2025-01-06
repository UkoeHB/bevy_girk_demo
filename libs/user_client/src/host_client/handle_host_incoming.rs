use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use bevy_simplenet::ClientReport;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_connection_change(
    In(report): In<ClientReport>,
    mut c: Commands,
    mut status: ReactResMut<ConnectionStatus>,
)
{
    match report {
        ClientReport::Connected => *status.get_mut(&mut c) = ConnectionStatus::Connected,
        ClientReport::Disconnected | ClientReport::ClosedByServer(_) | ClientReport::ClosedBySelf => {
            *status.get_mut(&mut c) = ConnectionStatus::Connecting;
            c.add(prep_fncall((), handle_connection_lost));
        }
        ClientReport::IsDead(aborted_reqs) => {
            *status.get_mut(&mut c) = ConnectionStatus::Dead;
            for aborted_req in aborted_reqs {
                c.add(prep_fncall(aborted_req, handle_request_aborted));
            }
            c.add(prep_fncall((), handle_connection_lost));
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_host_incoming(world: &mut World)
{
    while let Some(client_event) = world.resource_mut::<HostUserClient>().next() {
        match client_event {
            HostUserClientEvent::Report(report) => world.syscall(report, handle_connection_change),
            HostUserClientEvent::Msg(msg) => match msg {
                HostToUserMsg::LobbyState { lobby } => world.syscall(lobby, handle_lobby_state_update),
                HostToUserMsg::LobbyLeave { id } => world.syscall(id, handle_lobby_leave),
                HostToUserMsg::PendingLobbyAckRequest { id } => {
                    world.syscall(id, handle_pending_lobby_ack_request)
                }
                HostToUserMsg::PendingLobbyAckFail { id } => world.syscall(id, handle_pending_lobby_ack_fail),
                HostToUserMsg::GameStart { id, connect, start } => {
                    world.syscall((id, connect, start), handle_game_start)
                }
                HostToUserMsg::GameAborted { id } => world.syscall(id, handle_game_aborted),
                HostToUserMsg::GameOver { id, report } => world.syscall((id, report), handle_game_over),
            },
            HostUserClientEvent::Response(resp, request_id) => match resp {
                HostToUserResponse::LobbySearchResult(result) => {
                    world.syscall((request_id, result), handle_lobby_search_result);
                }
                HostToUserResponse::LobbyJoin { lobby } => {
                    world.syscall((request_id, lobby), handle_lobby_join);
                }
                HostToUserResponse::ConnectToken { id, connect } => {
                    world.syscall((request_id, id, connect), handle_connect_token);
                }
            },
            HostUserClientEvent::Ack(request_id) => world.syscall(request_id, handle_request_ack),
            HostUserClientEvent::Reject(request_id) => world.syscall(request_id, handle_request_rejected),
            HostUserClientEvent::SendFailed(request_id) => world.syscall(request_id, handle_send_failed),
            HostUserClientEvent::ResponseLost(request_id) => world.syscall(request_id, handle_response_lost),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
