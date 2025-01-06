use bevy_girk_utils::*;
use bevy_replicon::prelude::ChannelKind;
use serde::{Deserialize, Serialize};

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Reasons a game request may be rejected
#[derive(Debug, Serialize, Deserialize)]
pub enum RejectionReason
{
    ModeMismatch,
    Invalid,
    None,
}

//-------------------------------------------------------------------------------------------------------------------

/// Messages that can be sent out of the game.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameMsg
{
    RequestRejected
    {
        reason: RejectionReason,
        request: ClientRequest,
    },
    CurrentGameState(GameState),
}

impl IntoChannelKind for GameMsg
{
    fn into_event_type(&self) -> ChannelKind
    {
        match &self {
            Self::RequestRejected { .. } => SendUnordered.into(),
            Self::CurrentGameState(_) => SendOrdered.into(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
