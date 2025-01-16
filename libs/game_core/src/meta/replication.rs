use bevy::prelude::*;
use bevy_replicon::prelude::AppRuleExt;

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
        app.replicate::<PlayerId>()
            .replicate::<PlayerName>()
            .replicate::<PlayerScore>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
