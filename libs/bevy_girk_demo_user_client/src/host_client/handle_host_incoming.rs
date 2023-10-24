//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_host_incoming(
    mut commands : Commands,
    client       : Res<HostUserClient>,
){
    while let Some(server_val) = client.next_val()
    {
        match server_val
        {
            HostUserServerVal::Msg(msg) =>
            {
                match msg
                {
                    HostToUserMsg::LobbyState{ lobby } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, lobby, handle_lobby_state_update));
                    }
                    HostToUserMsg::LobbyLeave{ id } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, id, handle_lobby_leave));
                    }
                    HostToUserMsg::PendingLobbyAckRequest{ id } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, id, handle_pending_lobby_ack_request));
                    }
                    HostToUserMsg::PendingLobbyAckFail{ id } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, id, handle_pending_lobby_ack_fail));
                    }
                    HostToUserMsg::GameStart{ id, connect } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, (id, connect), handle_game_start));
                    }
                    HostToUserMsg::GameAborted{ id } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, id, handle_game_aborted));
                    }
                    HostToUserMsg::GameOver{ id, report } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, (id, report), handle_game_over));
                    }
                }
            }
            HostUserServerVal::Response(resp, request_id) =>
            {
                match resp
                {
                    HostToUserResponse::LobbySearchResult{ request, lobbies } =>
                    {
                        commands.add(
                            move |world: &mut World| syscall(world, (request_id, request, lobbies), handle_lobby_search_result)
                        );
                    }
                    HostToUserResponse::LobbyJoin{ lobby } =>
                    {
                        commands.add(move |world: &mut World| syscall(world, (request_id, lobby), handle_lobby_join));
                    }
                }
            }
            HostUserServerVal::Ack(request_id) =>
            {
                commands.add(move |world: &mut World| syscall(world, request_id, handle_request_ack));
            }
            HostUserServerVal::Reject(request_id) =>
            {
                commands.add(move |world: &mut World| syscall(world, request_id, handle_request_rejected));
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
