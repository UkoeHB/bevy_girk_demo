use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_user_client_utils::*;
use bevy_girk_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn clear_pending_request<M>(
    commands: &mut Commands,
    request_id: u64,
    result: Result<(Entity, &React<PendingRequest>), M>,
) -> bool
{
    let Ok((entity, pending_req)) = result else {
        return false;
    };
    if pending_req.id() != request_id {
        return false;
    }
    let Some(mut entity_commands) = commands.get_entity(entity) else {
        return false;
    };

    entity_commands.remove::<React<PendingRequest>>();

    true
}

//-------------------------------------------------------------------------------------------------------------------

fn remove_failed_pending_request(
    commands: &mut Commands,
    request_id: u64,
    pending_requests: &Query<(Entity, &React<PendingRequest>)>,
)
{
    for (entity, pending_req) in pending_requests.iter() {
        if pending_req.id() != request_id {
            continue;
        }
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_connection_lost(
    mut c: Commands,
    mut lobby_display: ReactResMut<LobbyDisplay>,
    mut ack_request: ReactResMut<AckRequestData>,
    mut starter: ReactResMut<ClientStarter>,
)
{
    // clear lobby display if hosted
    if lobby_display.is_hosted() {
        lobby_display.get_mut(&mut c).clear();
    }

    // clear ack request
    if ack_request.is_set() {
        ack_request.get_mut(&mut c).clear();
    }

    // clear starter
    // - We clear the starter to avoid a situation where a game over/abort is not received from the host server
    //   since it's disconnected, so the starter never gets cleared. When we reconnect to the host server, we will
    //   get a fresh game start package which will be used to reconnect the game automatically (if needed).
    if starter.has_starter() {
        starter.get_mut(&mut c).force_clear();
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_lobby_state_update(
    In(lobby_data): In<LobbyData>,
    mut c: Commands,
    mut lobby_display: ReactResMut<LobbyDisplay>,
)
{
    let lobby_id = lobby_data.id;
    tracing::info!("lobby state update received for lobby {lobby_id}");

    // check if the updated state matches the current lobby
    if lobby_display.lobby_id() != Some(lobby_id) {
        tracing::warn!("ignoring lobby state update for unknown lobby {lobby_id}");
        return;
    }

    // update lobby state
    if let Err(_) = lobby_display
        .get_mut(&mut c)
        .try_set(lobby_data, LobbyType::Hosted)
    {
        tracing::error!("ignoring lobby state update for invalid lobby {lobby_id}");
        return;
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_lobby_leave(
    In(lobby_id): In<u64>,
    mut c: Commands,
    mut lobby_display: ReactResMut<LobbyDisplay>,
)
{
    tracing::info!("lobby leave received for lobby {lobby_id}");

    // check if the lobby matches the current lobby
    if lobby_display.lobby_id() != Some(lobby_id) {
        tracing::warn!("ignoring leave lobby for unknown lobby {lobby_id}");
        return;
    }

    // clear lobby state
    if lobby_display.is_set() {
        lobby_display.get_mut(&mut c).clear();
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_pending_lobby_ack_request(In(lobby_id): In<u64>, mut c: Commands)
{
    tracing::info!("pending lobby ack request received for lobby {lobby_id}");

    // send ack request event
    c.react().broadcast(AckRequest { lobby_id });
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_pending_lobby_ack_fail(
    In(lobby_id): In<u64>,
    mut c: Commands,
    mut ack_request: ReactResMut<AckRequestData>,
)
{
    tracing::info!("pending lobby ack fail received for lobby {lobby_id}");

    // check if the lobby matches the ack request lobby
    if ack_request.get() != Some(lobby_id) {
        tracing::warn!("ignoring pending lobby ack fail for unknown lobby {lobby_id}");
        return;
    }

    // clear ack request
    if ack_request.is_set() {
        ack_request.get_mut(&mut c).clear();
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_game_start(
    In((game_id, token, start)): In<(u64, ServerConnectToken, GameStartInfo)>,
    mut c: Commands,
    mut lobby_display: ReactResMut<LobbyDisplay>,
    mut ack_request: ReactResMut<AckRequestData>,
    mut monitor: ReactResMut<ClientMonitor>,
    mut starter: ReactResMut<ClientStarter>,
    configs: Res<ClientLaunchConfigs>,
)
{
    tracing::info!("game start info received for game {game_id}");

    // clear lobby state
    if lobby_display.is_set() {
        lobby_display.get_mut(&mut c).clear();
    }

    // clear ack request
    if ack_request.is_set() {
        ack_request.get_mut(&mut c).clear();
    }

    // update starter
    // - do this before checking the current game in case the starter was cleared due to a host server disconnect
    let config = configs.multiplayer.clone();
    let launcher = move |monitor: &mut ClientMonitor, token: ServerConnectToken| {
        launch_multiplayer_client(monitor, config.clone(), token, start.clone());
    };
    starter
        .get_mut(&mut c)
        .set(game_id, GameLocation::Hosted, launcher);

    // if we are already running this game, then send in the connect token
    // - it's likely that the game client was also disconnected, but failed to request a new connect token since
    //   the user client was disconnected
    let current_game_id = monitor.game_id();
    if (Some(game_id) == current_game_id) && monitor.is_running() {
        tracing::info!("received game start for game {game_id} that is already running, sending new connect \
            token to game");
        monitor.get_noreact().send_token(token);
        return;
    }

    // kill the existing game client
    let monitor = monitor.get_mut(&mut c);
    if let Some(id) = current_game_id {
        monitor.kill(id);
    }

    // launch the game
    starter.get_noreact().start(monitor, token).unwrap();
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_game_aborted(
    In(game_id): In<u64>,
    mut c: Commands,
    mut starter: ReactResMut<ClientStarter>,
    config: Option<Res<ClientFwConfig>>,
)
{
    tracing::info!("game {game_id} aborted by host server");

    // clear starter
    if starter.has_starter() {
        starter.get_mut(&mut c).clear(game_id);
    }

    // force-close existing game
    //todo: display a message to user informing them a game was aborted
    if let Some(config) = config {
        if config.game_id() == game_id {
            c.queue(ClientInstanceCommand::Abort);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_game_over(
    In((game_id, game_over_report)): In<(u64, GameOverReport)>,
    mut c: Commands,
    mut starter: ReactResMut<ClientStarter>,
)
{
    tracing::info!("game over report received for game {game_id}");

    // clear starter
    if starter.has_starter() {
        starter.get_mut(&mut c).clear(game_id);
    }

    // send out report for use by the app
    c.react().broadcast(game_over_report);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_lobby_search_result(
    In((request_id, result)): In<(u64, LobbySearchResult)>,
    mut c: Commands,
    pending_search: Query<(Entity, &React<PendingRequest>), With<LobbySearch>>,
    mut lobby_page: ReactResMut<LobbyPage>,
)
{
    tracing::info!("lobby search result received for request {request_id}");

    // clear pending request
    if !clear_pending_request(&mut c, request_id, pending_search.get_single()) {
        tracing::warn!("ignoring unexpected lobby search result for request {request_id}");
        return;
    }

    // update lobby page
    if let Err(_) = lobby_page.get_mut(&mut c).try_set(result) {
        tracing::error!("failed setting new lobby page, lobbies are invalid");
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_lobby_join(
    In((request_id, lobby_data)): In<(u64, LobbyData)>,
    mut c: Commands,
    mut lobby_display: ReactResMut<LobbyDisplay>,
    pending_lobby_join: Query<(Entity, &React<PendingRequest>), Or<(With<JoinLobby>, With<MakeLobby>)>>,
)
{
    let lobby_id = lobby_data.id;
    tracing::info!("join lobby received for lobby {lobby_id} and request {request_id}");

    // clear pending request
    for (entity, pending_req) in pending_lobby_join.iter() {
        if pending_req.id() != request_id {
            continue;
        }
        let Some(mut entity_commands) = c.get_entity(entity) else {
            continue;
        };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }

    // populate lobby display
    if let Err(_) = lobby_display
        .get_mut(&mut c)
        .try_set(lobby_data, LobbyType::Hosted)
    {
        tracing::error!("ignoring join lobby for invalid lobby {lobby_id}");
        return;
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_connect_token(
    In((request_id, game_id, token)): In<(u64, u64, ServerConnectToken)>,
    mut c: Commands,
    mut monitor: ReactResMut<ClientMonitor>,
    mut starter: ReactResMut<ClientStarter>,
    token_request: Query<(Entity, &React<PendingRequest>), With<ConnectTokenRequest>>,
    pending_button: Query<(Entity, &React<PendingRequest>), With<ReconnectorButton>>,
)
{
    tracing::info!("connect token received for request {request_id}");

    // check if we are currently tracking this game
    if starter.game_id() != Some(game_id) {
        tracing::warn!("ignoring connect token for unknown game {game_id}");
        return;
    }

    // clear corresponding request
    let _ = clear_pending_request(&mut c, request_id, token_request.get_single());
    let _ = clear_pending_request(&mut c, request_id, pending_button.get_single());

    // handle the token
    match monitor.is_running() {
        // if the game is running, send the token to the game
        true => {
            tracing::info!("sending new connect token into game {game_id}");
            monitor.get_noreact().send_token(token);
        }
        // if not running, relaunch the game
        false if starter.has_starter() => {
            tracing::info!("restarting game {game_id} with new connect token");
            if starter
                .get_noreact()
                .start(monitor.get_mut(&mut c), token)
                .is_err()
            {
                tracing::error!("failed using starter to reconnect game {game_id}");
            }
        }
        // not running and not expecting a token
        _ => {
            // This can happen if a connect token is requested by a running game client, then the client is closed
            // and another token is requested via the reconnect button. If the game client reopens then shuts down
            // due to a game over before the second token arrives, then this branch will execute non-erroneously.
            tracing::warn!("received unexpected connect token for game {game_id}");
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_request_ack(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request ack received for request {request_id}");

    // find pending request and remove it
    //todo: consider allowing a custom callback for acks
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_request_rejected(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request rejection received for request {request_id}");

    // find pending request and remove it
    //todo: consider allowing a custom callback for rejections
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_send_failed(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request {request_id} send failed");

    // find pending request and remove it
    //todo: consider allowing a custom callback for failed sends
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_response_lost(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request {request_id} response lost");

    // find pending request and remove it
    //todo: consider allowing a custom callback for lost responses
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_request_aborted(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request {request_id} aborted");

    // find pending request and remove it
    //todo: consider allowing a custom callback for lost responses
    remove_failed_pending_request(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------
