use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_client_instance::*;
use bevy_girk_user_client_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn handle_token_req(
    In(game_id): In<u64>,
    mut c: Commands,
    client: Res<HostUserClient>,
    starter: ReactRes<ClientStarter>,
    request: Query<Entity, (With<ConnectTokenRequest>, Without<React<PendingRequest>>)>,
)
{
    // sanity check
    if Some(game_id) != starter.game_id() {
        tracing::warn!("ignoring connect token request for game {game_id}; client doesn't appear to be in \
            that game");
        return;
    }

    // check for existing request
    let Ok(target_entity) = request.get_single() else {
        tracing::error!("ignoring client's connect token request because a request is already pending");
        return;
    };

    // request new connect token
    let new_req = client.request(UserToHostRequest::GetConnectToken { id: game_id });

    // save request
    c.react()
        .insert(target_entity, PendingRequest::new(new_req));

    tracing::info!("requested new connect token from host server for game {game_id}");
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_ended(In(game_id): In<u64>, mut c: Commands, mut starter: ReactRes<ClientStarter>)
{
    tracing::info!("client instance ended for game {game_id}");
    starter.get_mut(&mut c).clear(game_id);
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_aborted(In(game_id): In<u64>, mut c: Commands, mut starter: ReactRes<ClientStarter>)
{
    tracing::warn!("client instance aborted for game {game_id}");
    starter.get_mut(&mut c).clear(game_id);
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_client_instance_reports(mut c: Commands, mut events: EventReader<ClientInstanceReport>)
{
    for report in events.read() {
        match report {
            ClientInstanceReport::RequestConnectToken(game_id) => c.syscall(game_id, handle_token_req),
            ClientInstanceReport::Ended(game_id) => c.syscall(game_id, handle_client_ended),
            ClientInstanceReport::Aborted(game_id) => c.syscall(game_id, handle_client_aborted),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct ClientInstanceReportPlugin;

impl Plugin for ClientInstanceReportPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(First, handle_client_instance_reports);
    }
}

//-------------------------------------------------------------------------------------------------------------------
