use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_attributes::*;
use serde::{Deserialize, Serialize};

use crate::ClientId;

//-------------------------------------------------------------------------------------------------------------------

/// Player id component (wraps the player's client id).
#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct PlayerId
{
    pub id: ClientId,
}

//-------------------------------------------------------------------------------------------------------------------

/// Player name component.
#[derive(Component, Default, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct PlayerName
{
    pub name: String,
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct PlayerScore
{
    /// Note: This is only `pub` for testing purposes.
    pub score: u32,
}

impl PlayerScore
{
    pub(crate) fn increment(&mut self)
    {
        self.score += 1;
    }
    pub fn score(&self) -> u32
    {
        self.score
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Players are entities with the components bundled here.
#[derive(Bundle)]
pub struct PlayerState
{
    /// Player id. Can be used to access player context from game context.
    pub id: PlayerId,
    /// Player name.
    pub name: PlayerName,
    /// Current player score
    pub score: PlayerScore,
    /// Players are replicated
    pub replicate: Replicated,
    /// Players have a visibility condition.
    pub visibility: VisibilityCondition,
}

impl Default for PlayerState
{
    fn default() -> Self
    {
        Self {
            id: PlayerId { id: 1 },
            name: Default::default(),
            score: Default::default(),
            replicate: Default::default(),
            visibility: vis![Global],
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
