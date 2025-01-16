use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use bevy_simplenet::ClientReport;

use super::*;

//-------------------------------------------------------------------------------------------------------------------

fn handle_connection_change(
    In(report): In<ClientReport>,
    mut c: Commands,
    mut status: ReactResMut<ConnectionStatus>,
)
{
    let status = status.get_mut(&mut c);
    match report {
        ClientReport::Connected => *status = ConnectionStatus::Connected,
        ClientReport::Disconnected | ClientReport::ClosedByServer(_) | ClientReport::ClosedBySelf => {
            *status = ConnectionStatus::Connecting;
            c.syscall((), handle_connection_lost);
        }
        ClientReport::IsDead(aborted_reqs) => {
            *status = ConnectionStatus::Dead;
            for aborted_req in aborted_reqs {
                c.syscall(aborted_req, handle_request_aborted);
            }
            c.syscall((), handle_connection_lost);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_host_incoming(w: &mut World)
{
    while let Some(client_event) = w.resource_mut::<HostUserClient>().next() {
        match client_event {
            HostUserClientEvent::Report(report) => w.syscall(report, handle_connection_change),
            HostUserClientEvent::Msg(msg) => match msg {
                HostToUserMsg::LobbyState { lobby } => w.syscall(lobby, handle_lobby_state_update),
                HostToUserMsg::LobbyLeave { id } => w.syscall(id, handle_lobby_leave),
                HostToUserMsg::PendingLobbyAckRequest { id } => w.syscall(id, handle_pending_lobby_ack_request),
                HostToUserMsg::PendingLobbyAckFail { id } => w.syscall(id, handle_pending_lobby_ack_fail),
                HostToUserMsg::GameStart { id, token, start } => w.syscall((id, token, start), handle_game_start),
                HostToUserMsg::GameAborted { id } => w.syscall(id, handle_game_aborted),
                HostToUserMsg::GameOver { id, report } => w.syscall((id, report), handle_game_over),
            },
            HostUserClientEvent::Response(resp, request_id) => match resp {
                HostToUserResponse::LobbySearchResult(result) => {
                    w.syscall((request_id, result), handle_lobby_search_result);
                }
                HostToUserResponse::LobbyJoin { lobby } => {
                    w.syscall((request_id, lobby), handle_lobby_join);
                }
                HostToUserResponse::ConnectToken { id, token } => {
                    w.syscall((request_id, id, token), handle_connect_token);
                }
            },
            HostUserClientEvent::Ack(request_id) => w.syscall(request_id, handle_request_ack),
            HostUserClientEvent::Reject(request_id) => w.syscall(request_id, handle_request_rejected),
            HostUserClientEvent::SendFailed(request_id) => w.syscall(request_id, handle_send_failed),
            HostUserClientEvent::ResponseLost(request_id) => w.syscall(request_id, handle_response_lost),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(SystemSet, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub(super) struct HandleHostIncomingSet;

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct HostIncomingPlugin;

impl Plugin for HostIncomingPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(First, handle_host_incoming.in_set(HandleHostIncomingSet));
    }
}

//-------------------------------------------------------------------------------------------------------------------
