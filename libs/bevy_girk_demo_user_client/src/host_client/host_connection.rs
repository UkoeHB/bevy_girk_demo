//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    mut rcommands     : ReactCommands,
    client            : Res<HostUserClient>,
    mut status        : ResMut<ReactRes<ConnectionStatus>>,
    mut pending_reset : ResMut<PendingLobbyReset>,
){
    let mut new_status = **status;

    while let Some(connection_report) = client.next_report()
    {
        match connection_report
        {
            bevy_simplenet::ClientReport::Connected         =>
            {
                new_status = ConnectionStatus::Connected;
                let Ok(signal) = client.request(UserToHostRequest::ResetLobby) else { continue; };
                pending_reset.set(signal.id());
            },
            bevy_simplenet::ClientReport::Disconnected      |
            bevy_simplenet::ClientReport::ClosedByServer(_) |
            bevy_simplenet::ClientReport::ClosedBySelf      => new_status = ConnectionStatus::Connecting,
            bevy_simplenet::ClientReport::IsDead            => new_status = ConnectionStatus::Dead,
        }
    }

    if new_status != **status { *status.get_mut(&mut rcommands) = new_status; }
}

//-------------------------------------------------------------------------------------------------------------------
