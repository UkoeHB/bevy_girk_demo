use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_client_fw::ClientFwConfig;
use bevy_girk_client_instance::ClientInstanceCommand;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_wiring_common::ServerConnectToken;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn pending_request_succeeded<M: Component>(
    c: &mut Commands,
    request_id: u64,
    param: &PendingRequestParam<M>,
) -> bool
{
    let Some((entity, req_signal)) = param.request() else { return false };
    let Some(mut ec) = c.get_entity(entity) else { return false };
    if req_signal.id() != request_id {
        return false;
    }

    ec.remove::<React<PendingRequest>>();
    ec.react().entity_event(entity, RequestSucceeded);

    true
}

//-------------------------------------------------------------------------------------------------------------------

fn pending_request_failed(
    c: &mut Commands,
    request_id: u64,
    pending_requests: &Query<(Entity, &React<PendingRequest>)>,
)
{
    for (entity, pending_req) in pending_requests.iter() {
        if pending_req.id() != request_id {
            continue;
        }
        let Some(mut ec) = c.get_entity(entity) else { continue };

        ec.remove::<React<PendingRequest>>();
        ec.react().entity_event(entity, RequestFailed);
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
    tracing::warn!("host server connection lost...");

    // clear lobby display if hosted by backend
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
    mut starter: ReactResMut<ClientStarter>,
    mut cached: ResMut<CachedConnectToken>,
    config: Option<Res<ClientFwConfig>>,
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
    starter.get_mut(&mut c).set(game_id, start);

    // if we are already running this game, then send in the connect token
    // - it's likely that the game client was also disconnected, but failed to request a new connect token since
    //   the user client was disconnected
    let current_game_id = config.map(|c| c.game_id());
    if Some(game_id) == current_game_id {
        tracing::info!("received game start for game {game_id} that is already running, discarding new connect \
            token");
        return;
    }

    // kill the existing game client
    if current_game_id.is_some() {
        c.queue(ClientInstanceCommand::Abort);
    }

    // prep to launch the game once the app is ready
    cached.set(game_id, token);
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
    //todo: display a popup message to user informing them a game was aborted
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

    // note: we don't send ClientInstanceCommand::End because the client may want to continue displaying things
    // during game-over
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_lobby_search_result(
    In((request_id, result)): In<(u64, LobbySearchResult)>,
    mut c: Commands,
    pending_search: PendingRequestParam<LobbySearch>,
    mut lobby_page: ReactResMut<LobbyPage>,
)
{
    tracing::info!("lobby search result received; request={request_id}");

    // clear pending request
    if !pending_request_succeeded(&mut c, request_id, &pending_search) {
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
    join_lobby_request: PendingRequestParam<JoinLobby>,
    make_lobby_request: PendingRequestParam<MakeLobby>,
)
{
    let lobby_id = lobby_data.id;
    tracing::info!("join lobby received for lobby {lobby_id}; request={request_id}");

    // clear pending request
    if pending_request_succeeded(&mut c, request_id, &join_lobby_request) {
    } else if pending_request_succeeded(&mut c, request_id, &make_lobby_request) {
    }

    // populate lobby display
    if let Err(err) = lobby_display
        .get_mut(&mut c)
        .try_set(lobby_data, LobbyType::Hosted)
    {
        tracing::error!("ignoring attempt to join lobby {lobby_id}; err={:?}", err);
        return;
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_connect_token(
    In((request_id, game_id, token)): In<(u64, u64, ServerConnectToken)>,
    mut c: Commands,
    starter: ReactRes<ClientStarter>,
    token_request: PendingRequestParam<ConnectTokenRequest>,
    mut cached: ResMut<CachedConnectToken>,
)
{
    tracing::info!("connect token received for game {game_id}; request={request_id}");

    // check if we are currently tracking this game
    if starter.game_id() != Some(game_id) {
        tracing::warn!("ignoring connect token for unknown game {game_id}");
        return;
    }

    // clear corresponding request
    let _ = pending_request_succeeded(&mut c, request_id, &token_request);

    // cache token so the game can be restarted as needed
    cached.set(game_id, token);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_request_ack(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request ack received; request={request_id}");

    //todo: consider allowing a custom callback for acks
    pending_request_failed(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_request_rejected(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request rejection received; request={request_id}");

    //todo: consider allowing a custom callback for rejections
    pending_request_failed(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_send_failed(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request {request_id} send failed");

    //todo: consider allowing a custom callback for failed sends
    pending_request_failed(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_response_lost(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request {request_id} response lost");

    //todo: consider allowing a custom callback for lost responses
    pending_request_failed(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn handle_request_aborted(
    In(request_id): In<u64>,
    mut commands: Commands,
    pending_requests: Query<(Entity, &React<PendingRequest>)>,
)
{
    tracing::info!("request {request_id} aborted");

    //todo: consider allowing a custom callback for lost responses
    pending_request_failed(&mut commands, request_id, &pending_requests);
}

//-------------------------------------------------------------------------------------------------------------------
