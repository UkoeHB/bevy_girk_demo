//local shortcuts

//third-party shortcuts
use serde::{Serialize, Deserialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Player inputs that can be sent to the game.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PlayerInput
{
    /// Click a button.
    ClickButton,
    /// Placeholder
    None
}

//-------------------------------------------------------------------------------------------------------------------

/// Requests that can be sent to the game.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum GameRequest
{
    /// Request the current game mode.
    GameModeRequest,
    /// Player input.
    PlayerInput(PlayerInput)
}

//-------------------------------------------------------------------------------------------------------------------
