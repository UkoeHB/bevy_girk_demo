//local shortcuts

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

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn status_to_string(status: ConnectionStatus) -> &'static str
{
    match status
    {
        ConnectionStatus::Connecting => "Connecting...",
        ConnectionStatus::Connected  => "Connected",
        ConnectionStatus::Dead       => "DEAD",
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_connection_changes(
    client     : Res<HostUserClient>,
    mut status : ResMut<ConnectionStatus>
){
    while let Some(connection_report) = client.next_report()
    {
        match connection_report
        {
            bevy_simplenet::ClientReport::Connected         =>
            {
                *status = ConnectionStatus::Connected;
                let _ = client.request(UserToHostRequest::ResetLobby);
            },
            bevy_simplenet::ClientReport::Disconnected      |
            bevy_simplenet::ClientReport::ClosedByServer(_) |
            bevy_simplenet::ClientReport::ClosedBySelf      => *status = ConnectionStatus::Connecting,
            bevy_simplenet::ClientReport::IsDead            => *status = ConnectionStatus::Dead,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
