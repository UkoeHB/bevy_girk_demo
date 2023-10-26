//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Tracks the request id of the last `ResetLobby` sent to the host server.
///
/// When a `ResetLobby` is acknowledged, the client can confidently reset its current lobby-tracking state.
#[derive(Resource, Debug)]
pub(crate) struct PendingLobbyReset
{
    /// pending reset request id
    current: Option<u64>,
}

impl PendingLobbyReset
{
    pub(crate) fn set(&mut self, request_id: u64)
    {
        self.current = Some(request_id);
    }

    pub(crate) fn clear(&mut self)
    {
        self.current = None;
    }

    pub(crate) fn matches_request(&self, request_id: u64) -> bool
    {
        match self.current
        {
            Some(current_req_id) => current_req_id == request_id,
            None                 => false,
        }
    }
}

impl Default for PendingLobbyReset { fn default() -> Self { Self{ current: None } } }

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn PendingLobbyResetPlugin(app: &mut App)
{
    app
        .insert_resource(PendingLobbyReset::default())
        ;
}

//-------------------------------------------------------------------------------------------------------------------
