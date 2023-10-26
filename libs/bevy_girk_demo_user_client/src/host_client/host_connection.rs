//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Copy, Clone, Eq, PartialEq, Debug)]
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

pub(crate) fn handle_connection_changes(
    client            : Res<HostUserClient>,
    mut status        : ResMut<ConnectionStatus>,
    mut pending_reset : ResMut<PendingLobbyReset>,
){
    while let Some(connection_report) = client.next_report()
    {
        match connection_report
        {
            bevy_simplenet::ClientReport::Connected         =>
            {
                *status = ConnectionStatus::Connected;
                let Ok(signal) = client.request(UserToHostRequest::ResetLobby) else { continue; };
                pending_reset.set(signal.id());
            },
            bevy_simplenet::ClientReport::Disconnected      |
            bevy_simplenet::ClientReport::ClosedByServer(_) |
            bevy_simplenet::ClientReport::ClosedBySelf      => *status = ConnectionStatus::Connecting,
            bevy_simplenet::ClientReport::IsDead            => *status = ConnectionStatus::Dead,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
