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
    pub client_id: ClientId,
    /// Player score during the game.
    pub score: PlayerScore,
}

//-------------------------------------------------------------------------------------------------------------------

/// Report emitted at the end of a game.
#[derive(Serialize, Deserialize)]
pub struct ClickGameOverReport
{
    /// The last game tick that elapsed before this report was created.
    pub final_game_tick: Tick,

    /// Each player's individual report.
    pub player_reports: Vec<ClickPlayerReport>,
}

//-------------------------------------------------------------------------------------------------------------------
