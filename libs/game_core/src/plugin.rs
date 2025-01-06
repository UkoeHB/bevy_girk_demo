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
use bevy_girk_game_fw::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Game plugin. Depends on [`GameFwPlugin`], which should be added by the game factory.
pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add(GameReplicationPlugin)
            .add(GameSetsPlugin)
            .add(GameSetupPlugin)
            .add(GameStatePlugin)
            .add(GameTickPlugin)
            .configure_sets(
                Update,
                (
                    GameStateUpdateSet,
                    TickUpdateSet,
                )
                    .chain()
                    .in_set(GameLogicSet::Admin)
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
