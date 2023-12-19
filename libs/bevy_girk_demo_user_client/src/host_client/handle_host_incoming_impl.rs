//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn clear_pending_request<E>(
    commands   : &mut Commands,
    request_id : u64,
    result     : Result<(Entity, &React<PendingRequest>), E>
) -> bool
{
    let Ok((entity, pending_req)) = result else { return false; };
    if pending_req.id() != request_id { return false; }
    let Some(mut entity_commands) = commands.get_entity(entity) else { return false; };

    entity_commands.remove::<React<PendingRequest>>();

    true
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn remove_failed_pending_request(
    commands         : &mut Commands,
    request_id       : u64,
    pending_requests : &Query<(Entity, &React<PendingRequest>)>
){
    for (entity, pending_req) in pending_requests.iter()
    {
        if pending_req.id() != request_id { continue; }
        let Some(mut entity_commands) = commands.get_entity(entity) else { continue; };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_connection_lost(
    mut rcommands     : ReactCommands,
    mut lobby_display : ReactResMut<LobbyDisplay>,
    mut ack_request   : ReactResMut<AckRequestData>,
    mut reconnector   : ReactResMut<GameReconnector>,
){
    // clear lobby display if hosted
    if lobby_display.is_hosted() { lobby_display.get_mut(&mut rcommands).clear(); }

    // clear ack request
    if ack_request.is_set() { ack_request.get_mut(&mut rcommands).clear(); }

    // clear reconnector
    // - We clear the reconnector to avoid a situation where a game over/abort is not received from the host server
    //   since it's disconnected, so the reconnector never gets cleared. When the we reconnect to the host server, we
    //   will get a fresh game start package which will be used to reconnect the game automatically (if needed).
    if reconnector.can_reconnect() { reconnector.get_mut(&mut rcommands).force_clear_if(LobbyType::Hosted); }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_state_update(
    In(lobby_data)    : In<LobbyData>,
    mut rcommands     : ReactCommands,
    mut lobby_display : ReactResMut<LobbyDisplay>,
){
    let lobby_id = lobby_data.id;
    tracing::info!(lobby_id, "lobby state update received");

    // check if the updated state matches the current lobby
    if lobby_display.lobby_id() != Some(lobby_id)
    {
        tracing::warn!(lobby_id, "ignoring lobby state update for unknown lobby");
        return;
    }

    // update lobby state
    if let Err(_) = lobby_display.get_mut(&mut rcommands).try_set(lobby_data, LobbyType::Hosted)
    {
        tracing::error!(lobby_id, "ignoring lobby state update for invalid lobby");
        return;
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_leave(
    In(lobby_id)      : In<u64>,
    mut rcommands     : ReactCommands,
    mut lobby_display : ReactResMut<LobbyDisplay>,
){
    tracing::info!(lobby_id, "lobby leave received");

    // check if the lobby matches the current lobby
    if lobby_display.lobby_id() != Some(lobby_id)
    {
        tracing::warn!(lobby_id, "ignoring leave lobby for unknown lobby");
        return;
    }

    // clear lobby state
    if lobby_display.is_set() { lobby_display.get_mut(&mut rcommands).clear(); }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_pending_lobby_ack_request(
    In(lobby_id)  : In<u64>,
    mut rcommands : ReactCommands,
){
    tracing::info!(lobby_id, "pending lobby ack request received");

    // send ack request event
    rcommands.send(AckRequest{ lobby_id });
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_pending_lobby_ack_fail(
    In(lobby_id)    : In<u64>,
    mut rcommands   : ReactCommands,
    mut ack_request : ReactResMut<AckRequestData>,
){
    tracing::info!(lobby_id, "pending lobby ack fail received");

    // check if the lobby matches the ack request lobby
    if ack_request.get() != Some(lobby_id)
    {
        tracing::warn!(lobby_id, "ignoring pending lobby ack fail for unknown lobby");
        return;
    }

    // clear ack request
    if ack_request.is_set() { ack_request.get_mut(&mut rcommands).clear(); }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_start(
    In((
        lobby_id,
        token,
        start,
    ))                : In<(u64, ServerConnectToken, GameStartInfo)>,
    mut rcommands     : ReactCommands,
    mut lobby_display : ReactResMut<LobbyDisplay>,
    mut ack_request   : ReactResMut<AckRequestData>,
    mut game_monitor  : ReactResMut<GameMonitor>,
    mut reconnector   : ReactResMut<GameReconnector>,
){
    tracing::info!(lobby_id, "game start info received");

    // clear lobby state
    if lobby_display.is_set() { lobby_display.get_mut(&mut rcommands).clear(); }

    // clear ack request
    if ack_request.is_set() { ack_request.get_mut(&mut rcommands).clear(); }

    // update reconnector
    // - do this before checking the current game in case the reconnector was cleared due to a host server disconnect
    let launcher =
        move |monitor: &mut GameMonitor, token: ServerConnectToken|
        {
            launch_multiplayer_game(monitor, lobby_id, token, start.clone());
        };
    reconnector.get_mut(&mut rcommands).set(lobby_id, LobbyType::Hosted, launcher.clone());

    // if we are already running this game, then send in the connect token
    // - it's likely that the game client was also disconnected, but failed to request a new connect token since the
    //   user client was disconnected
    let current_game_id = game_monitor.game_id();
    if (Some(lobby_id) == current_game_id) && game_monitor.is_running()
    {
        tracing::info!(lobby_id, "received game start for game that is already running, sending new connect token to game");
        panic!("sending tokens into game client not yet supported");
        //game_monitor.get_mut_noreact().send_token(token);
        //return;
    }

    // kill the existing game client
    let game_monitor = game_monitor.get_mut(&mut rcommands);
    if let Some(id) = current_game_id { game_monitor.kill(id); }

    // launch the game
    (launcher)(game_monitor, token);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_aborted(
    In(lobby_id)     : In<u64>,
    mut rcommands    : ReactCommands,
    mut game_monitor : ReactResMut<GameMonitor>,
    mut reconnector  : ReactResMut<GameReconnector>,
){
    tracing::info!(lobby_id, "game aborted by host server");

    // force-close existing game
    //todo: display a message to user informing them a game was aborted
    if game_monitor.is_running() { game_monitor.get_mut(&mut rcommands).kill(lobby_id); }

    // clear reconnector
    if reconnector.can_reconnect() { reconnector.get_mut(&mut rcommands).clear(lobby_id); }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_over(
    In((
        lobby_id,
        game_over_report
    ))              : In<(u64, GameOverReport)>,
    mut rcommands   : ReactCommands,
    mut reconnector : ReactResMut<GameReconnector>,
){
    tracing::info!(lobby_id, "game over report received");

    // clear reconnector
    if reconnector.can_reconnect() { reconnector.get_mut(&mut rcommands).clear(lobby_id); }

    // send game over report data event
    //todo: do something with the report
    rcommands.send(game_over_report);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_search_result(
    In((
        request_id,
        result
    ))             : In<(u64, LobbySearchResult)>,
    mut rcommands  : ReactCommands,
    pending_search : Query<(Entity, &React<PendingRequest>), With<LobbySearch>>,
    mut lobby_page : ReactResMut<LobbyPage>,
){
    tracing::info!(request_id, "lobby search result received");

    // clear pending request
    if !clear_pending_request(rcommands.commands(), request_id, pending_search.get_single())
    { tracing::warn!(request_id, "ignoring unexpected lobby search result"); return; }

    // update lobby page
    if let Err(_) = lobby_page.get_mut(&mut rcommands).try_set(result)
    { tracing::error!("failed setting new lobby page, lobbies are invalid"); }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_join(
    In((request_id, lobby_data)) : In<(u64, LobbyData)>,
    mut rcommands                : ReactCommands,
    mut lobby_display            : ReactResMut<LobbyDisplay>,
    pending_lobby_join           : Query<(Entity, &React<PendingRequest>), Or<(With<JoinLobby>, With<MakeLobby>)>>
){
    let lobby_id = lobby_data.id;
    tracing::info!(request_id, lobby_id, "join lobby received");

    // clear pending request
    for (entity, pending_req) in pending_lobby_join.iter()
    {
        if pending_req.id() != request_id { continue; }
        let Some(mut entity_commands) = rcommands.commands().get_entity(entity) else { continue; };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }

    // populate lobby display
    if let Err(_) = lobby_display.get_mut(&mut rcommands).try_set(lobby_data, LobbyType::Hosted)
    {
        tracing::error!(lobby_id, "ignoring join lobby for invalid lobby");
        return;
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_connect_token(
    In((
        request_id,
        game_id,
        token
    ))               : In<(u64, u64, ServerConnectToken)>,
    mut rcommands    : ReactCommands,
    mut game_monitor : ReactResMut<GameMonitor>,
    mut reconnector  : ReactResMut<GameReconnector>,
    pending_button   : Query<(Entity, &React<PendingRequest>), With<ReconnectorButton>>,
){
    tracing::info!(request_id, "connect token received");

    // check if we are currently tracking this game
    if reconnector.game_id() != Some(game_id) { tracing::warn!("ignoring connect token for unknown game"); return; }

    // clear reconnector request
    let _ = clear_pending_request(rcommands.commands(), request_id, pending_button.get_single());

    // handle the token
    match game_monitor.is_running()
    {
        // if the game is running, send the token to the game
        true =>
        {
            tracing::info!(game_id, "sending new connect token into game");
            panic!("sending token to game is not yet supported");  //todo
            //game_monitor.get_mut_noreact().send_token(token);
        }
        // if not running, relaunch the game
        false if reconnector.can_reconnect() =>
        {
            tracing::info!(game_id, "restarting game with new connect token");
            if reconnector.get_mut_noreact().reconnect(game_monitor.get_mut(&mut rcommands), token).is_err()
            { tracing::error!(game_id, "failed using reconnector to reconnect a game"); }
        }
        // not running and not expecting a token
        _ =>
        {
            // This can happen if a connect token is requested by a running game client, then the client is closed
            // and another token is requested via the reconnect button. If the game client reopens then shuts down
            // due to a game over before the second token arrives, then this branch will execute non-erroneously.
            tracing::warn!(game_id, "received unexpected connect token");
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_request_ack(
    In(request_id)   : In<u64>,
    mut commands     : Commands,
    pending_requests : Query<(Entity, &React<PendingRequest>)>
){
    tracing::info!(request_id, "request ack received");

    // find pending request and remove it
    //todo: consider allowing a custom callback for acks
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_request_rejected(
    In(request_id)   : In<u64>,
    mut commands     : Commands,
    pending_requests : Query<(Entity, &React<PendingRequest>)>
){
    tracing::info!(request_id, "request rejection received");

    // find pending request and remove it
    //todo: consider allowing a custom callback for rejections
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_send_failed(
    In(request_id)   : In<u64>,
    mut commands     : Commands,
    pending_requests : Query<(Entity, &React<PendingRequest>)>
){
    tracing::info!(request_id, "request send failed");

    // find pending request and remove it
    //todo: consider allowing a custom callback for failed sends
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_response_lost(
    In(request_id)   : In<u64>,
    mut commands     : Commands,
    pending_requests : Query<(Entity, &React<PendingRequest>)>
){
    tracing::info!(request_id, "request response lost");

    // find pending request and remove it
    //todo: consider allowing a custom callback for lost responses
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_request_aborted(
    In(request_id)   : In<u64>,
    mut commands     : Commands,
    pending_requests : Query<(Entity, &React<PendingRequest>)>
){
    tracing::info!(request_id, "request aborted");

    // find pending request and remove it
    //todo: consider allowing a custom callback for lost responses
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------
