use bevy_girk_utils::*;
use bevy_replicon::prelude::EventType;
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
    CurrentGameMode(GameMode),
}

impl IntoEventType for GameMsg
{
    fn into_event_type(&self) -> EventType
    {
        match &self {
            Self::RequestRejected { .. } => SendUnordered.into(),
            Self::CurrentGameMode(_) => SendOrdered.into(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
