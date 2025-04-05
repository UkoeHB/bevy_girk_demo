use bevy_girk_utils::*;
use bevy_replicon::prelude::Channel;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------------------------

/// Player inputs that can be sent to the game.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PlayerInput
{
    /// Click a button.
    ClickButton,
}

impl IntoChannel for PlayerInput
{
    fn into_event_type(&self) -> Channel
    {
        match &self {
            Self::ClickButton => SendUnordered.into(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Requests that can be sent to the game.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum ClientRequest
{
    /// Request the current game mode.
    GetGameState,
    /// Player input.
    PlayerInput(PlayerInput),
}

impl IntoChannel for ClientRequest
{
    fn into_event_type(&self) -> Channel
    {
        match &self {
            Self::GetGameState => SendOrdered.into(),
            Self::PlayerInput(input) => input.into_event_type(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
