//local shortcuts

//third-party shortcuts
use bevy_cobweb::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(ReactResource, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ConnectionStatus
{
    Connecting,
    Connected,
    Dead,
}

impl ConnectionStatus
{
    pub(crate) fn to_str(&self) -> &'static str
    {
        match self
        {
            ConnectionStatus::Connecting => "Connecting...",
            ConnectionStatus::Connected  => "Connected",
            ConnectionStatus::Dead       => "DEAD",
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
