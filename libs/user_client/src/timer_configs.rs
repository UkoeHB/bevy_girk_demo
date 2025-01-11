use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimerConfigs
{
    /// Timeout for reconstructing the host-user client when it dies.
    pub host_reconstruct_loop_ms: u64,
    /// Timeout for re-requesting connect tokens when token requests fail.
    pub token_request_loop_ms: u64,
    /// Timeout after which an ack request expires.
    pub ack_request_timeout_ms: u64,
    /// Amount of time the user client ack request should 'shave off' the ack request timeout for displaying to
    /// users.
    pub ack_request_timer_buffer_ms: u64,
    /// Refresh interval for the lobby list.
    pub lobby_list_refresh_ms: u64,
}

//-------------------------------------------------------------------------------------------------------------------
