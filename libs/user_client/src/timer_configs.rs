use bevy::prelude::*;
use bevy_girk_user_client_utils::*;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimerConfigs
{
    /// Timeout after which an ack request expires.
    pub ack_request_timeout_ms: u64,
    /// Amount of time the user client ack request should 'shave off' the ack request timeout for displaying to
    /// users.
    pub ack_request_timer_buffer_ms: u64,
    /// Refresh interval for the lobby list.
    pub lobby_list_refresh_ms: u64,
}

//-------------------------------------------------------------------------------------------------------------------
