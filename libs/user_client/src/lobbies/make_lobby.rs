use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::{HostUserClient, UserToHostRequest};
use bevy_girk_utils::ser_msg;
use bevy_girk_wiring_common::ConnectionType;
use wiring_backend::{ClickLobbyConfig, ClickLobbyContents, ClickLobbyMemberType};

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Convert state to lobby contents.
///
/// Panics if not single-player.
fn single_player_lobby(owner_id: u128, data: &MakeLobbyData) -> ClickLobbyContents
{
    if !data.is_single_player() {
        panic!("cannot convert make lobby data to lobby contents for multiplayer lobbies");
    }

    ClickLobbyContents {
        id: 0u64,
        owner_id,
        config: data.config.clone(),
        players: vec![(ConnectionType::Memory, owner_id)], // Must use memory connection type
        watchers: vec![],
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn make_local_lobby(
    mut c: Commands,
    client: Res<HostUserClient>,
    make_lobby: PendingRequestParam<MakeLobby>,
    mut lobby_display: ReactResMut<LobbyDisplay>,
    data: ReactRes<MakeLobbyData>,
)
{
    // do nothing if there is a pending request
    if make_lobby.has_request() {
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
    tracing::trace!(?data.member_type, ?data.config, "making a local lobby");
    lobby_display
        .get_mut(&mut c)
        .set(single_player_lobby(client.id(), &data), LobbyType::Local);

    // send event for UI updates
    c.react().broadcast(MadeLocalLobby);
}

//-------------------------------------------------------------------------------------------------------------------

fn send_make_lobby_request(
    mut c: Commands,
    client: Res<HostUserClient>,
    make_lobby: PendingRequestParam<MakeLobby>,
    mut data: ReactResMut<MakeLobbyData>,
)
{
    // get request entity
    // - do nothing if there is already a pending request
    if make_lobby.has_request() {
        tracing::warn!("ignoring make lobby request because a request is already pending");
        return;
    };

    // request to make a lobby
    // - note: do not log the password
    tracing::trace!(?data.member_type, ?data.config, "requesting to make lobby");

    let new_req = client.request(UserToHostRequest::MakeLobby {
        mcolor: data.member_type.into(),
        pwd: data.pwd.clone(),
        data: ser_msg(&data.config),
    });

    // save request
    let request = PendingRequest::new(new_req);
    make_lobby.add_request(&mut c, request);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_a_lobby(world: &mut World)
{
    match world.react_resource::<MakeLobbyData>().is_single_player() {
        true => world.syscall((), make_local_lobby),
        false => world.syscall((), send_make_lobby_request),
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the make lobby workflow.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
pub(crate) struct MakeLobbyData
{
    /// Cached member type.
    pub(crate) member_type: ClickLobbyMemberType,
    /// Cached password.
    pub(crate) pwd: String,
    /// Cached lobby config.
    pub(crate) config: ClickLobbyConfig,
}

impl MakeLobbyData
{
    pub(crate) fn is_single_player(&self) -> bool
    {
        self.config.is_single_player()
    }
}

impl Default for MakeLobbyData
{
    fn default() -> Self
    {
        Self {
            member_type: ClickLobbyMemberType::Player,
            pwd: String::default(),
            config: ClickLobbyConfig { max_players: 1, max_watchers: 0 },
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when a local lobby has been constructed.
pub(crate) struct MadeLocalLobby;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct MakeLobbyPlugin;

impl Plugin for MakeLobbyPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_react_resource::<MakeLobbyData>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
