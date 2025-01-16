//! Plugins for core game logic.
//!
//! PRECONDITION: plugin dependencies
//! - bevy_replicon::core::ReplicationCorePlugin
//!
//! PRECONDITION: the following must be initialized by the user
//! - Res<ClickGameInitializer>
//!
//! INTERFACE: for client core
//! - plugin GameReplicationPlugin must be added to the client core app

use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Game plugin. Depends on [`GameFwPlugin`], which should be added by the game factory.
pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(GameReplicationPlugin)
            .add_plugins(GameSetsPlugin)
            .add_plugins(GameSetupPlugin)
            .add_plugins(GameStatePlugin)
            .add_plugins(GameTickPlugin)
            .configure_sets(
                Update,
                (GameStateUpdateSet, TickUpdateSet)
                    .chain()
                    .in_set(GameLogicSet::Admin),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
