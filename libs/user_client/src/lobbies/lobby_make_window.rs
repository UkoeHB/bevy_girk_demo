use bevy::prelude::*;
use bevy_cobweb::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Convert window state to lobby contents.
///
/// Panics if not single-player.
fn single_player_lobby(owner_id: u128, window: &MakeLobbyWindow) -> ClickLobbyContents
{
    if !window.is_single_player() {
        panic!("cannot convert make lobby window to lobby contents for multiplayer lobbies");
    }

    ClickLobbyContents {
        id: 0u64,
        owner_id,
        config: window.config.clone(),
        players: vec![(ConnectionType::Memory, owner_id)], // Must use memory connection type
        watchers: vec![],
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when a local lobby has been constructed.
pub(crate) struct MadeLocalLobby;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_local_lobby(
    mut c: Commands,
    client: Res<HostUserClient>,
    make_lobby: Query<(), (With<MakeLobby>, With<React<PendingRequest>>)>,
    mut lobby_display: ReactResMut<LobbyDisplay>,
    window: ReactRes<MakeLobbyWindow>,
)
{
    // do nothing if there is a pending request
    if !make_lobby.is_empty() {
        tracing::warn!("ignoring make local lobby request because a multiplayer request is pending");
        return;
    };

    // do nothing if we are in a hosted lobby
    if lobby_display.is_hosted() {
        tracing::warn!("ignoring make local lobby request because we are in a multiplayer lobby");
        return;
    };

    // make a local lobby
    // - note: do not log the password
    tracing::trace!(?window.member_type, ?window.config, "making a local lobby");
    lobby_display
        .get_mut(&mut c)
        .set(single_player_lobby(client.id(), &window), LobbyType::Local);

    // send event for UI updates
    c.react().broadcast(MadeLocalLobby);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn send_make_lobby_request(
    mut c: Commands,
    client: Res<HostUserClient>,
    make_lobby: Query<Entity, (With<MakeLobby>, Without<React<PendingRequest>>)>,
    mut window: ReactResMut<MakeLobbyWindow>,
)
{
    // get request entity
    // - do nothing if there is already a pending request
    let Ok(target_entity) = make_lobby.get_single() else {
        tracing::warn!("ignoring make lobby request because a request is already pending");
        return;
    };

    // request to make a lobby
    // - note: do not log the password
    tracing::trace!(?window.member_type, ?window.config, "requesting to make lobby");

    let new_req = client.request(UserToHostRequest::MakeLobby {
        mcolor: window.member_type.into(),
        pwd: window.pwd.clone(),
        data: ser_msg(&window.config),
    });

    // save request
    let request = PendingRequest::new(new_req);
    c.react().insert(target_entity, request.clone());
    window.get_noreact().last_req = Some(request);
}

//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the make lobby window.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
pub(crate) struct MakeLobbyWindow
{
    /// Cached member type.
    pub(crate) member_type: ClickLobbyMemberType,
    /// Cached password.
    pub(crate) pwd: String,
    /// Cached lobby config.
    pub(crate) config: ClickLobbyConfig,

    /// Last request sent
    pub(crate) last_req: Option<PendingRequest>,
}

impl MakeLobbyWindow
{
    pub(crate) fn is_single_player(&self) -> bool
    {
        self.config.is_single_player()
    }
}

impl Default for MakeLobbyWindow
{
    fn default() -> Self
    {
        Self {
            member_type: ClickLobbyMemberType::Player,
            pwd: String::default(),
            config: ClickLobbyConfig { max_players: 1, max_watchers: 0 },
            last_req: None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct LobbyMakeWindowPlugin;

impl Plugin for LobbyMakeWindowPlugin
{
    fn build(&self, _app: &mut App)
    {
        app.init_react_resource::<MakeLobbyWindow>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
