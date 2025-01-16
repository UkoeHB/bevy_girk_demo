use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use wiring_backend::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn leave_current_lobby(
    mut c: Commands,
    client: Res<HostUserClient>,
    mut lobby: ReactResMut<LobbyDisplay>,
    leave_lobby: PendingRequestParam<LeaveLobby>,
)
{
    // check for existing request
    if leave_lobby.has_request() {
        tracing::warn!("ignoring leave lobby request because a request is already pending");
        return;
    }

    // check if we are in a lobby
    let Some(lobby_id) = lobby.lobby_id() else {
        tracing::error!("tried to leave lobby but we aren't in a lobby");
        return;
    };

    // leave the lobby
    match lobby.lobby_type() {
        None => {
            tracing::error!("tried to leave lobby but there is no lobby type");
            return;
        }
        Some(LobbyType::Local) => {
            // clear the lobby
            if lobby.is_set() {
                lobby.get_mut(&mut c).clear();
            }
        }
        Some(LobbyType::Hosted) => {
            // send leave request
            let new_req = client.request(UserToHostRequest::LeaveLobby { id: lobby_id });

            // save request
            leave_lobby.add_request(&mut c, new_req);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn start_current_lobby(
    mut c: Commands,
    client: Res<HostUserClient>,
    mut lobby: ReactResMut<LobbyDisplay>,
    starter: ReactRes<ClientStarter>,
    config: Option<Res<ClientFwConfig>>,
    launch_lobby: PendingRequestParam<LaunchLobby>,
)
{
    // check for existing request
    if launch_lobby.has_request() {
        tracing::warn!("ignoring start lobby request because a request is pending");
        return;
    }

    // check if we are in a lobby
    let Some(lobby_id) = lobby.lobby_id() else {
        tracing::error!("tried to start lobby but we aren't in a lobby");
        return;
    };

    // check if there is an existing game
    if config.is_some() {
        tracing::error!("tried to start lobby {} but we are currently in game {}", lobby.lobby_id(), config.game_id());
        return;
    };

    // launch the lobby
    match lobby.lobby_type() {
        None => {
            tracing::error!("tried to start lobby but there is no lobby type");
            return;
        }
        Some(LobbyType::Local) => {
            // clear lobby display
            let Some(lobby_contents) = lobby.get_mut(&mut c).clear() else {
                tracing::error!("lobby contents are missing in local lobby");
                return;
            };

            // check if hosted game is being set up/reconnected
            if starter.has_starter() {
                tracing::warn!("tried to start local lobby but a hosted game (id={}) is being set up",
                    starter.game_id().unwrap());
                return;
            }

            // prep launch pack
            let game_configs = make_click_game_configs();
            let Ok(launch_pack) = get_launch_pack(game_configs, lobby_contents) else {
                tracing::error!("failed getting launch pack for local player game");
                return;
            };

            // launch the game
            c.queue(ClientInstanceCommand::StartLocal(launch_pack));
        }
        Some(LobbyType::Hosted) => {
            // send launch reqeust
            let new_req = client.request(UserToHostRequest::LaunchLobbyGame { id: lobby_id });

            // save request
            launch_lobby.add_request(&mut c, new_req);
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum LobbyType
{
    /// On the local machine (single-player).
    Local,
    /// On a host server.
    Hosted,
}

//-------------------------------------------------------------------------------------------------------------------

/// Caches the currently-displayed lobby that the user is a member of.
#[derive(ReactResource, Debug, Default)]
pub(crate) struct LobbyDisplay
{
    current: Option<ClickLobbyContents>,
    lobby_type: Option<LobbyType>,
}

impl LobbyDisplay
{
    pub(crate) fn set(&mut self, contents: ClickLobbyContents, lobby_type: LobbyType)
    {
        self.current = Some(contents);
        self.lobby_type = Some(lobby_type);
    }

    /// Returns `Err` if lobby contents cannot be extracted from the lobby data.
    pub(crate) fn try_set(&mut self, new_lobby: LobbyData, lobby_type: LobbyType) -> Result<(), String>
    {
        self.current = Some(ClickLobbyContents::try_from(new_lobby)?);
        self.lobby_type = Some(lobby_type);

        Ok(())
    }

    pub(crate) fn clear(&mut self) -> Option<ClickLobbyContents>
    {
        let current = self.current.take();
        self.lobby_type = None;

        current
    }

    pub(crate) fn lobby_id(&self) -> Option<u64>
    {
        match &self.current {
            Some(data) => Some(data.id),
            None => None,
        }
    }

    pub(crate) fn get(&self) -> Option<&ClickLobbyContents>
    {
        self.current.as_ref()
    }

    pub(crate) fn lobby_type(&self) -> Option<LobbyType>
    {
        self.lobby_type
    }

    pub(crate) fn is_set(&self) -> bool
    {
        self.current.is_some()
    }

    pub(crate) fn is_local(&self) -> bool
    {
        self.lobby_type == Some(LobbyType::Local)
    }

    pub(crate) fn is_hosted(&self) -> bool
    {
        self.lobby_type == Some(LobbyType::Hosted)
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct LobbyDisplayPlugin;

impl Plugin for LobbyDisplayPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_react_resource::<LobbyDisplay>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
