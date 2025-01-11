use bevy::prelude::*;
use bevy_cobweb::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn send_join_lobby_request(
    mut c: Commands,
    client: Res<HostUserClient>,
    join_lobby: Query<Entity, (With<JoinLobby>, Without<React<PendingRequest>>)>,
    mut window: ReactResMut<JoinLobbyWindow>,
)
{
    // get request entity
    // - do nothing if there is already a pending request
    let Ok(target_entity) = join_lobby.get_single() else {
        tracing::warn!("ignoring join lobby request because a request is already pending");
        return;
    };

    // fail if there is no lobby
    let Some(lobby_contents) = &window.contents else {
        tracing::error!("lobby contents are missing for join lobby request");
        return;
    };

    // request to join the specified lobby
    // - note: do not log the password
    let lobby_id = lobby_contents.id;
    tracing::trace!(lobby_id, ?window.member_type, "requesting to join lobby");

    let new_req = client.request(UserToHostRequest::JoinLobby {
        id: lobby_id,
        mcolor: window.member_type.into(),
        pwd: window.pwd.clone(),
    });

    // save request
    let request = PendingRequest::new(new_req);
    c.react().insert(target_entity, request.clone());
    window.get_noreact().last_req = Some(request);
}

//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the join lobby window.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
pub(crate) struct JoinLobbyWindow
{
    /// Lobby contents.
    pub(crate) contents: Option<ClickLobbyContents>,

    /// Cached member type.
    pub(crate) member_type: ClickLobbyMemberType,
    /// Cached password.
    pub(crate) pwd: String,

    /// Last request sent
    pub(crate) last_req: Option<PendingRequest>,
}

impl Default for JoinLobbyWindow
{
    fn default() -> Self
    {
        Self {
            contents: None,
            member_type: ClickLobbyMemberType::Player,
            pwd: String::default(),
            last_req: None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct LobbyJoinWindowPlugin;

impl Plugin for LobbyJoinWindowPlugin
{
    fn build(&self, _app: &mut App)
    {
        app.init_react_resource::<JoinLobbyWindow>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
