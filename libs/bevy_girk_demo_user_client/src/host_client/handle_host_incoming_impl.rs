//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_state_update(
    In(lobby_data): In<LobbyData>,
){
    tracing::info!(lobby_data.id, "lobby state update received");

    //if in this lobby, update its state
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_leave(
    In(lobby_id): In<u64>,
){
    tracing::info!(lobby_id, "lobby leave received");

    //if in this lobby, leave
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_pending_lobby_ack_request(
    In(lobby_id): In<u64>,
){
    tracing::info!(lobby_id, "pending lobby ack request received");

    //display ack request
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_pending_lobby_ack_fail(
    In(lobby_id): In<u64>,
){
    tracing::info!(lobby_id, "pending lobby ack fail received");

    //close ack request window if it exists
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_start(
    In((lobby_id, _game_connect)): In<(u64, GameConnectInfo)>,
){
    tracing::info!(lobby_id, "game start info received");

    //launch a game
    tracing::error!(lobby_id, "game starting is not yet implemented");
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_aborted(
    In(lobby_id): In<u64>,
){
    tracing::info!(lobby_id, "game aborted by host server");

    //force-close existing game
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_over(
    In((lobby_id, game_over_report)): In<(u64, GameOverReport)>,
){
    tracing::info!(lobby_id, "game over report received");

    //display game over report
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_search_result(
    In((request_id, request, lobbies)): In<(u64, LobbySearchRequest, Vec<LobbyData>)>,
){
    tracing::info!(request_id, "lobby search result received");

    //if pending lobby search, update lobby list
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_join(
    In((request_id, lobby_data)): In<(u64, LobbyData)>,
){
    tracing::info!(request_id, lobby_data.id, "join lobby received");

    //if pending lobby join, populate lobby display
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_request_ack(
    In(request_id): In<u64>,
){
    tracing::info!(request_id, "request ack received");

    //reset lobby: if in a lobby, clear it
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_request_rejected(
    In(request_id): In<u64>,
){
    tracing::info!(request_id, "request rejection received");

    //reset lobby: if in a lobby, clear it
}

//-------------------------------------------------------------------------------------------------------------------
