use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::{HostUserClient, UserToHostRequest};
use wiring_backend::{ClickLobbyContents, ClickLobbyMemberType};

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn send_join_lobby_request(
    mut c: Commands,
    client: Res<HostUserClient>,
    join_lobby: PendingRequestParam<JoinLobby>,
    data: ReactRes<JoinLobbyData>,
)
{
    // get request entity
    // - do nothing if there is already a pending request
    if join_lobby.has_request() {
        tracing::warn!("ignoring join lobby request because a request is already pending");
        return;
    };

    // fail if there is no lobby
    let Some(lobby_contents) = &data.contents else {
        tracing::error!("lobby contents are missing for join lobby request");
        return;
    };

    // request to join the specified lobby
    // - note: do not log the password
    let lobby_id = lobby_contents.id;
    tracing::trace!(lobby_id, ?data.member_type, "requesting to join lobby");

    let new_req = client.request(UserToHostRequest::JoinLobby {
        id: lobby_id,
        mcolor: data.member_type.into(),
        pwd: data.pwd.clone(),
    });

    // save request
    let request = PendingRequest::new(new_req);
    join_lobby.add_request(&mut c, request);
}

//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the join lobby workflow.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
pub(crate) struct JoinLobbyData
{
    /// Lobby contents.
    pub(crate) contents: Option<ClickLobbyContents>,

    /// Cached member type.
    pub(crate) member_type: ClickLobbyMemberType,
    /// Cached password.
    pub(crate) pwd: String,
}

impl JoinLobbyData
{
    pub(crate) fn clear(&mut self)
    {
        *self = Self::default();
    }
}

impl Default for JoinLobbyData
{
    fn default() -> Self
    {
        Self {
            contents: None,
            member_type: ClickLobbyMemberType::Player,
            pwd: String::default(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct JoinLobbyPlugin;

impl Plugin for JoinLobbyPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_react_resource::<JoinLobbyData>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
