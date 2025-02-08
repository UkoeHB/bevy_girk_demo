//! Plugins for the core of a player client.
//!
//! PRECONDITION: plugin dependencies
//! - bevy_replicon::core::ReplicationCorePlugin
//!
//! PRECONDITION: the following must be initialized by the client manager
//! - Res<ClientInitializer>
//! - Res<Receiver<PlayerInput>>

use bevy::prelude::*;
use bevy_girk_client_fw::*;
use game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Request the current game mode.
fn request_game_state(mut sender: ClientSender)
{
    sender.send(ClientRequest::GetGameState);
}

//-------------------------------------------------------------------------------------------------------------------

pub struct ClientCorePlugin;

impl Plugin for ClientCorePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(GameReplicationPlugin)
            .add_plugins(ClientSetsPlugin)
            .add_plugins(ClientSetupPlugin)
            // For this demo we assume watcher clients will re-use the player skin, which depends on
            // `PlayerInputPlugin`. A different project may want to completely separate player and
            // watcher skins, in which case this plugin can go in a player-client-specific crate.
            .add_plugins(PlayerInputPlugin)
            .add_systems(OnEnter(ClientInitState::Done), request_game_state)
            .configure_sets(Update, PlayerInputSet.in_set(ClientLogicSet::Admin));
    }
}

//-------------------------------------------------------------------------------------------------------------------
