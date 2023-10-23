//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Player report for the game over report.
#[derive(Serialize, Deserialize)]
pub struct ClickPlayerReport
{
    /// Client id within the game.
    pub client_id: ClientIdType,
    /// Player score during the game.
    pub score: PlayerScore,
}

//-------------------------------------------------------------------------------------------------------------------

/// Report emitted at the end of a game.
#[derive(Serialize, Deserialize)]
pub struct ClickGameOverReport
{
    /// Total ticks that elapsed during the game.
    pub game_ticks: Ticks,

    /// Each player's individual report.
    pub player_reports: Vec<ClickPlayerReport>,
}

//-------------------------------------------------------------------------------------------------------------------
