use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::*;

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
    pub players: HashMap<ClientId, PlayerState>,
    /// Watchers.
    pub watchers: HashSet<ClientId>,
}

//-------------------------------------------------------------------------------------------------------------------
