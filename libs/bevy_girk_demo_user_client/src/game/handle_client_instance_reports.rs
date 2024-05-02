//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_client_instance::*;
use bevy_girk_user_client_utils::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_request_connect_token(
    mut c   : Commands,
    client  : Res<HostUserClient>,
    monitor : ReactRes<ClientMonitor>,
    request : Query<Entity, (With<ConnectTokenRequest>, Without<React<PendingRequest>>)>,
){
    // sanity check
    if !monitor.is_running() { tracing::warn!("ignoring connect token request from non-running client"); return; }

    // check for existing request
    let Ok(target_entity) = request.get_single()
    else { tracing::error!("ignoring client's connect token request because a request is already pending"); return };

    // game id
    let Some(game_id) = monitor.game_id() else { tracing::error!("running client monitor is missing its game id"); return; };

    // request new connect token
    let new_req = client.request(UserToHostRequest::GetConnectToken{ id: game_id });

    // save request
    c.react().insert(target_entity, PendingRequest::new(new_req));

    tracing::info!(game_id, "requested new connect token from host server");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_client_aborted(In(game_id): In<u64>)
{
    tracing::warn!(game_id, "client instance aborted");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_client_instance_reports(world: &mut World)
{
    while let Some(report) = world.react_resource_mut_noreact::<ClientMonitor>().next_report()
    {
        match report
        {
            ClientInstanceReport::RequestConnectToken => syscall(world, (), handle_request_connect_token),
            ClientInstanceReport::Aborted(game_id)    => syscall(world, game_id, handle_client_aborted),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
