use bevy::prelude::*;
use bevy_replicon::prelude::*;
use game_core::*;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum ClientType
{
    Player,
    Watcher,
}

//-------------------------------------------------------------------------------------------------------------------

/// Static information in a client app.
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct ClientContext
{
    /// This client's id
    client_id: ClientId,
    /// This client's type.
    client_type: ClientType,

    /// The game duration config.
    duration_config: GameDurationConfig,
}

impl ClientContext
{
    /// New context
    pub fn new(client_id: ClientId, client_type: ClientType, duration_config: GameDurationConfig)
        -> ClientContext
    {
        ClientContext { client_id, client_type, duration_config }
    }

    pub fn id(&self) -> ClientId
    {
        self.client_id
    }
    pub fn client_type(&self) -> ClientType
    {
        self.client_type
    }
    pub fn duration_config(&self) -> &GameDurationConfig
    {
        &self.duration_config
    }
}

//-------------------------------------------------------------------------------------------------------------------
