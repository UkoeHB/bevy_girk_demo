//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Data used to initialize a client app on startup.
///
/// This resource is consumed during initialization.
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct ClientInitializer
{
    /// The client's context.
    pub context: ClientContext,
}

//-------------------------------------------------------------------------------------------------------------------
