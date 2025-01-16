use bevy::prelude::*;
use bevy_cobweb::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn refresh_lobby_list(
    mut c: Commands,
    client: Res<HostUserClient>,
    lobby_search: PendingRequestParam<LobbySearch>,
    lobby_page_req: ReactRes<LobbyPageRequest>,
)
{
    // do nothing if there is already a pending lobby search
    if lobby_search.has_request() {
        tracing::debug!("ignoring lobby search request because a search is already pending");
        return;
    }

    // re-request the last-requested lobby page
    tracing::trace!("refreshing lobby list");
    let new_req = client.request(UserToHostRequest::LobbySearch(lobby_page_req.get().clone()));
    lobby_search.add_request(&mut c, new_req);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn request_lobby_list_now(
    mut c: Commands,
    client: Res<HostUserClient>,
    lobby_search: PendingRequestParam<LobbySearch>,
    mut lobby_page_req: ReactResMut<LobbyPageRequest>,
)
{
    // do nothing if there is already a pending lobby search
    if lobby_search.has_request() {
        tracing::debug!("ignoring lobby search request because a search is already pending");
        return;
    }

    // make request
    // - we request the highest-possible lobby id in order to get the youngest available lobby
    let req = LobbySearchRequest::PageOlder { youngest_id: u64::MAX, num: LOBBY_LIST_SIZE as u16 };

    // send request
    tracing::trace!("requesting lobby list: now");
    let new_req = client.request(UserToHostRequest::LobbySearch(req.clone()));

    // save request
    lobby_page_req.get_mut(&mut c).set(req);
    lobby_search.add_request(&mut c, new_req);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn request_lobby_list_next_newer(
    mut c: Commands,
    client: Res<HostUserClient>,
    lobby_search: PendingRequestParam<LobbySearch>,
    mut lobby_page_req: ReactResMut<LobbyPageRequest>,
    lobby_page: ReactRes<LobbyPage>,
)
{
    // do nothing if there is already a pending lobby search
    if lobby_search.has_request() {
        tracing::debug!("ignoring lobby search request because a search is already pending");
        return;
    }

    // make request
    let oldest_id = lobby_page
        .get()
        .get(0) // youngest currently-displayed lobby
        .map_or(
            // this branch may execute if lobbies in the last requested page are all removed by server updates, or
            // if the returned page is empty
            match lobby_page_req.get() {
                LobbySearchRequest::LobbyId(id) => id.saturating_sub(1u64),
                LobbySearchRequest::PageNewer { oldest_id, num } => oldest_id.saturating_add(*num as u64),
                LobbySearchRequest::PageOlder { youngest_id, num: _ } => youngest_id.saturating_add(1u64),
            },
            // next page starts at lobby younger than our current youngest
            |contents| contents.id.saturating_add(1u64),
        );

    let req = LobbySearchRequest::PageNewer { oldest_id, num: LOBBY_LIST_SIZE as u16 };

    // send request
    tracing::trace!("requesting lobby list: next newer");
    let new_req = client.request(UserToHostRequest::LobbySearch(req.clone()));

    // save request
    lobby_page_req.get_mut(&mut c).set(req);
    lobby_search.add_request(&mut c, new_req);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn request_lobby_list_next_older(
    mut c: Commands,
    client: Res<HostUserClient>,
    lobby_search: PendingRequestParam<LobbySearch>,
    mut lobby_page_req: ReactResMut<LobbyPageRequest>,
    lobby_page: ReactRes<LobbyPage>,
)
{
    // do nothing if there is already a pending lobby search
    if lobby_search.has_request() {
        tracing::debug!("ignoring lobby search request because a search is already pending");
        return;
    };

    // make request
    let youngest_id = lobby_page
        .get()
        .get(lobby_page.len().saturating_sub(1)) // oldest currently-displayed lobby
        .map_or(
            // this branch may execute if lobbies in the last requested page are all removed by server updates, or
            // if the returned page is empty
            match lobby_page_req.get() {
                LobbySearchRequest::LobbyId(id) => id.saturating_sub(1u64),
                LobbySearchRequest::PageNewer { oldest_id, num: _ } => oldest_id.saturating_sub(1u64),
                LobbySearchRequest::PageOlder { youngest_id, num } => youngest_id.saturating_sub(*num as u64),
            },
            // next page starts at lobby older than our current oldest
            |contents| contents.id.saturating_sub(1u64),
        );

    let req = LobbySearchRequest::PageOlder { youngest_id, num: LOBBY_LIST_SIZE as u16 };

    // send request
    tracing::trace!("requesting lobby list: next older");
    let new_req = client.request(UserToHostRequest::LobbySearch(req.clone()));

    // save request
    lobby_page_req.get_mut(&mut c).set(req);
    lobby_search.add_request(&mut c, new_req);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn request_lobby_list_oldest(
    mut c: Commands,
    client: Res<HostUserClient>,
    lobby_search: PendingRequestParam<LobbySearch>,
    mut lobby_page_req: ReactResMut<LobbyPageRequest>,
)
{
    // do nothing if there is already a pending lobby search
    if lobby_search.has_request() {
        tracing::warn!("ignoring lobby search request because a request is already pending");
        return;
    };

    // make request
    // - we request the lowest-possible lobby id in order to get the oldest available lobby
    let req = LobbySearchRequest::PageNewer { oldest_id: 0u64, num: LOBBY_LIST_SIZE as u16 };

    // send request
    tracing::trace!("requesting lobby list: oldest");
    let new_req = client.request(UserToHostRequest::LobbySearch(req.clone()));

    // save request
    lobby_page_req.get_mut(&mut c).set(req);
    lobby_search.add_request(&mut c, new_req);
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct LobbyListPlugin;

impl Plugin for LobbyListPlugin
{
    fn build(&self, app: &mut App)
    {
        let timer_configs = app.world.resource::<TimerConfigs>();
        let refresh = Duration::from_millis(timer_configs.lobby_list_refresh_ms);

        app.add_systems(
            PreUpdate,
            refresh_lobby_list
                // refresh the list automatically if:
                // - in client state
                // - viewing play section
                // - connected to host
                // - on timer OR just connected to host (note: test timer first to avoid double-refresh when timer
                //   is saturated) OR the lobby display was just changed OR the user just toggled to the play
                //   section
                .run_if(in_state(ClientAppState::Client))
                .run_if(|menu_section: Res<MenuContentSection>| *menu_section == MenuContentSection::Play)
                .run_if(|status: ReactRes<ConnectionStatus>| *status == ConnectionStatus::Connected)
                .run_if(
                    on_timer(refresh)
                        .or(|status: ReactRes<ConnectionStatus>| status.is_changed())
                        .or(|display: ReactRes<LobbyDisplay>| display.is_changed())
                        // Changed + section Play (filtered above)
                        .or(|menu_section: Res<MenuContentSection>| menu_section.is_changed()),
                ),
        );
    }
}

//-------------------------------------------------------------------------------------------------------------------
