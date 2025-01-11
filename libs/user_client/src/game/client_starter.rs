use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_client_instance::ClientInstanceCommand;
use bevy_girk_game_instance::GameStartInfo;
use bevy_girk_wiring_common::ServerConnectToken;

//-------------------------------------------------------------------------------------------------------------------

fn clear_starter(mut c: Commands, mut starter: ReactResMut<ClientStarter>, config: Option<Res<ClientFwConfig>>)
{
    let Some(config) = config else { return };
    starter.get_mut(&mut c).clear(config.game_id());
}

//-------------------------------------------------------------------------------------------------------------------

/// Game-starting only occurs in ClientAppState::Client, so we need a separate system to deal with it.
fn try_start_game(mut c: Commands, mut cached: ResMut<CachedConnectToken>, mut starter: ReactResMut<ClientStarter>)
{
    let Some(game_id) = cached.game_id.take() else { return };
    let Some(token) = cached.token.take() else { return };
    if Some(game_id) != starter.game_id() {
        tracing::warn!("discarding cached connect token, starter game id {} doesn't match cached game id {}",
            starter.game_id(), game_id);
    }
    if let Err(err) = starter.get_mut(&mut c).start(&mut c, token) {
        tracing::warn!("failed starting game client; err={err:?}");
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Default)]
pub(crate) struct CachedConnectToken
{
    game_id: Option<u64>,
    token: Option<ServerConnectToken>,
}

impl CachedConnectToken
{
    pub(crate) fn set(&mut self, game_id: u64, token: ServerConnectToken)
    {
        self.game_id = Some(game_id);
        self.token = Some(token);
    }

    pub(crate) fn has_token(&self) -> bool
    {
        self.token.is_some()
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Facilitates starting and restarting a game client.
///
/// This is a reactive resource for ease of use in a GUI app.
#[derive(ReactResource, Default)]
pub(crate) struct ClientStarter
{
    game_id: u64,
    /// Cached start info for re-use when reconnecting to a hosted game.
    start_info: Option<GameStartInfo>,
}

impl ClientStarter
{
    /// Set the starter.
    ///
    /// This will over-write the existing starter.
    pub(crate) fn set(&mut self, game_id: u64, start_info: GameStartInfo)
    {
        self.game_id = game_id;
        self.start_info = Some(start_info);
    }

    /// Check if there is a starter
    pub(crate) fn has_starter(&self) -> bool
    {
        self.start_info.is_some()
    }

    /// Gets the game id for the client that can be started.
    ///
    /// Returns `None` if [`Self::has_starter()`] is false.
    pub(crate) fn game_id(&self) -> Option<u64>
    {
        if !self.has_starter() {
            return None;
        }
        Some(self.game_id)
    }

    /// Tries to send [`ClientInstanceCommand::Start`].
    ///
    /// Returns an error if there is no registered starter.
    pub(crate) fn start(&self, c: &mut Commands, token: ServerConnectToken) -> Result<(), String>
    {
        let Some(start_info) = &self.start_info else {
            return Err("no start info".into());
        };
        c.queue(ClientInstanceCommand::Start(token, start_info.clone()));
        Ok(())
    }

    /// Clear the starter if it matches the given game id.
    pub(crate) fn clear(&mut self, game_id: u64)
    {
        if self.game_id != game_id {
            return;
        }
        self.start_info = None;
    }

    /// Clears the starter regardless of the current game id.
    ///
    /// This is useful as a fall-back when [`Self::clear()`] is not possible because the game id is unknown. For
    /// example, if your user client becomes disconnected from the host server and you expect to *maybe*
    /// receive a new game-start package when you reconnect.
    pub(crate) fn force_clear(&mut self)
    {
        self.start_info = None;
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Add a [`ClientStarter`] to your app.
pub(super) struct ClientStarterPlugin;

impl Plugin for ClientStarterPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_react_resource::<ClientStarter>()
            .init_resource::<CachedConnectToken>()
            .add_systems(OnEnter(ClientState::GameOver), clear_starter)
            .add_systems(
                Last,
                try_start_game
                    .run_if(in_state(ClientAppState::Client))
                    .run_if(in_state(LoadState::Done)),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
