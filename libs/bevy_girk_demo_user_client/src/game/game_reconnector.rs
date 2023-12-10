//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_fn_plugin::bevy_plugin;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Stores a reconnector for reconnecting to an ongoing game.
#[derive(ReactResource)]
pub(crate) struct GameReconnector
{
    game_id     : u64,
    lobby_type  : LobbyType,
    reconnector : Option<Box<dyn FnMut(&mut GameMonitor) + Send + Sync + 'static>>,
}

impl GameReconnector
{
    /// Set the reconnector.
    ///
    /// This will over-write the existing reconnector.
    pub(crate) fn set(
        &mut self,
        game_id: u64,
        lobby_type: LobbyType,
        reconnector: impl FnMut(&mut GameMonitor) + Send + Sync + 'static
    ){
        self.game_id     = game_id;
        self.lobby_type  = lobby_type;
        self.reconnector = Some(Box::new(reconnector));
    }

    /// Check if there is a reconnector
    pub(crate) fn can_reconnect(&self) -> bool
    {
        self.reconnector.is_some()
    }

    /// Try to reconnect.
    pub(crate) fn reconnect(&mut self, monitor: &mut GameMonitor)
    {
        let Some(reconnector) = &mut self.reconnector else { return; };
        (reconnector)(monitor);
    }

    /// Clear the reconnector if it contains the given game id.
    pub(crate) fn clear(&mut self, game_id: u64)
    {
        if self.game_id != game_id { return; }
        self.reconnector = None;
    }

    /// Clear the reconnector if it matches the given lobby type.
    pub(crate) fn force_clear_if(&mut self, lobby_type: LobbyType)
    {
        if self.lobby_type != lobby_type { return; }
        self.reconnector = None;
    }
}

impl Default for GameReconnector
{
    fn default() -> Self
    {
        Self{ game_id: 0u64, lobby_type: LobbyType::Local, reconnector: None }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn GameReconnectorPlugin(app: &mut App)
{
    app.insert_react_resource(GameReconnector::default());
}

//-------------------------------------------------------------------------------------------------------------------
