use bevy_girk_utils::*;
use bevy_replicon::prelude::ChannelKind;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------------------------

/// Player inputs that can be sent to the game.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PlayerInput
{
    /// Click a button.
    ClickButton,
}

impl IntoChannelKind for PlayerInput
{
    fn into_event_type(&self) -> ChannelKind
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
    GetGameMode,
    /// Player input.
    PlayerInput(PlayerInput),
}

impl IntoChannelKind for ClientRequest
{
    fn into_event_type(&self) -> ChannelKind
    {
        match &self {
            Self::GetGameMode => SendOrdered.into(),
            Self::PlayerInput(input) => input.into_event_type(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
