//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_fw::*;

//standard shortcuts
use std::collections::{HashMap, HashSet};

//-------------------------------------------------------------------------------------------------------------------

/// Data used on startup to initialize a game app.
///
/// This resource is consumed during initialization.
#[derive(Resource)]
pub struct ClickGameInitializer
{
    /// Game context.
    pub game_context: ClickGameContext,
    /// Player states.
    pub players: HashMap<ClientIdType, PlayerState>,
    /// Watchers.
    pub watchers: HashSet<ClientIdType>,
}

//-------------------------------------------------------------------------------------------------------------------
