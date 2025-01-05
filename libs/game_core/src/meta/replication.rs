use bevy::prelude::*;
use bevy_girk_utils::*;
use bevy_replicon_repair::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Initializes all game components that may be replicated (including game framework components).
///
/// Depends on `bevy_replicon::replication_core::ReplicationCorePlugin`.
pub struct GameReplicationPlugin;

impl Plugin for GameReplicationPlugin
{
    fn build(&self, app: &mut App)
    {
        app.replicate_repair::<PlayerId>()
            .replicate_repair_react::<PlayerName>()
            .replicate_repair_react::<PlayerScore>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
