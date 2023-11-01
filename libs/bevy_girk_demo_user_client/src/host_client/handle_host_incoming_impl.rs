//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_lobby_reset(
    mut rcommands     : ReactCommands,
    client            : Res<HostUserClient>,
    mut lobby_display : ResMut<ReactRes<LobbyDisplay>>,
    mut ack_request   : ResMut<ReactRes<AckRequest>>,
    mut pending_reset : ResMut<PendingLobbyReset>,
    lobby_page        : Query<(Entity, &LobbyPageRequest), With<LobbySearch>>,
){
    // clear pending reset
    pending_reset.clear();

    // clear lobby display if hosted
    if lobby_display.is_hosted() { lobby_display.get_mut(&mut rcommands).clear() };

    // clear ack request
    if ack_request.is_set() { ack_request.get_mut(&mut rcommands).clear(); }

    // re-request the last-requested lobby page
    let (entity, lobby_page_req) = lobby_page.single();
    rerequest_latest_lobby_page(&mut rcommands, &client, entity, &lobby_page_req);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_state_update(
    In(lobby_data)    : In<LobbyData>,
    mut rcommands     : ReactCommands,
    mut lobby_display : ResMut<ReactRes<LobbyDisplay>>,
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
    mut lobby_display : ResMut<ReactRes<LobbyDisplay>>,
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
    In(lobby_id)    : In<u64>,
    mut rcommands   : ReactCommands,
    mut ack_request : ResMut<ReactRes<AckRequest>>,
){
    tracing::info!(lobby_id, "pending lobby ack request received");

    // update ack request
    ack_request.get_mut(&mut rcommands).set(lobby_id);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_pending_lobby_ack_fail(
    In(lobby_id)    : In<u64>,
    mut rcommands   : ReactCommands,
    mut ack_request : ResMut<ReactRes<AckRequest>>,
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
    In((lobby_id, _game_connect)) : In<(u64, GameConnectInfo)>,
    mut rcommands                 : ReactCommands,
    mut lobby_display             : ResMut<ReactRes<LobbyDisplay>>,
    mut ack_request               : ResMut<ReactRes<AckRequest>>,
){
    tracing::info!(lobby_id, "game start info received");

    // clear lobby state
    if lobby_display.is_set() { lobby_display.get_mut(&mut rcommands).clear(); }

    // clear ack request
    if ack_request.is_set() { ack_request.get_mut(&mut rcommands).clear(); }

    //todo: launch a game
    tracing::error!(lobby_id, "game starting is not yet implemented");
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_aborted(
    In(lobby_id): In<u64>,
){
    tracing::info!(lobby_id, "game aborted by host server");

    //todo: force-close existing game
    tracing::error!(lobby_id, "game aborting is not yet implemented");
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_over(
    In((
        lobby_id,
        game_over_report
    ))            : In<(u64, GameOverReport)>,
    mut rcommands : ReactCommands,
){
    tracing::info!(lobby_id, "game over report received");

    // send game over report data event
    rcommands.send(game_over_report);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_search_result(
    In((
        request_id,
        _original_request,
        lobbies
    ))                 : In<(u64, LobbySearchRequest, Vec<LobbyData>)>,
    mut rcommands      : ReactCommands,
    mut pending_search : Query<(Entity, &React<PendingRequest>), With<LobbySearch>>,
    mut lobby_page     : ResMut<ReactRes<LobbyPage>>,
){
    tracing::info!(request_id, "lobby search result received");

    // clear pending request
    let Ok((entity, pending_req)) = pending_search.get_single_mut() else { return; };
    if pending_req.id() != request_id { return; }
    let Some(mut entity_commands) = rcommands.commands().get_entity(entity) else { return; };

    entity_commands.remove::<React<PendingRequest>>();

    // update lobby page
    lobby_page.get_mut(&mut rcommands).set(lobbies);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_lobby_join(
    In((request_id, lobby_data)) : In<(u64, LobbyData)>,
    mut rcommands                : ReactCommands,
    mut lobby_display            : ResMut<ReactRes<LobbyDisplay>>,
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

//todo: branch on the request id if more ack-style requests are added
pub(crate) fn handle_request_ack(
    In(request_id) : In<u64>,
    world          : &mut World,
){
    tracing::info!(request_id, "request ack received");

    if world.resource::<PendingLobbyReset>().matches_request(request_id)
    {
        syscall(world, (), handle_lobby_reset);
        return;
    }

    tracing::warn!(request_id, "ignoring ack for unknown request");
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_request_rejected(
    In(request_id)   : In<u64>,
    mut commands     : Commands,
    pending_reset    : Res<PendingLobbyReset>,
    pending_requests : Query<(Entity, &React<PendingRequest>)>
){
    tracing::info!(request_id, "request rejection received");

    // find pending request and remove it
    //todo: consider allowing a custom callback for rejections
    for (entity, pending_req) in pending_requests.iter()
    {
        if pending_req.id() != request_id { continue; }
        let Some(mut entity_commands) = commands.get_entity(entity) else { continue; };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }

    // handle rejected lobby reset (rejection means we are in-game)
    if pending_reset.matches_request(request_id)
    {
        commands.add(|world: &mut World| syscall(world, (), handle_lobby_reset));
    }
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
    for (entity, pending_req) in pending_requests.iter()
    {
        if pending_req.id() != request_id { continue; }
        let Some(mut entity_commands) = commands.get_entity(entity) else { continue; };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }
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
    for (entity, pending_req) in pending_requests.iter()
    {
        if pending_req.id() != request_id { continue; }
        let Some(mut entity_commands) = commands.get_entity(entity) else { continue; };

        entity_commands.remove::<React<PendingRequest>>();
        break;
    }
}

//-------------------------------------------------------------------------------------------------------------------
