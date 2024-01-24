//local shortcuts
use bevy_girk_demo_client_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_client_fw::*;
use iyes_progress::*;

//standard shortcuts
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
struct LoadingStartTime(Duration);

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn reset_loading_time_start(time: Res<Time>, mut start: ResMut<LoadingStartTime>)
{
    start.0 = time.elapsed();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Hacky timer for delaying initialization.
fn initialization_timer(time: Res<Time>, start: Res<LoadingStartTime>) -> Progress
{
    if time.elapsed() < start.0 + Duration::from_millis(500)
    {
        Progress{ done: 0, total: 2 }
    }
    else if time.elapsed() < start.0 + Duration::from_millis(1000)
    {
        Progress{ done: 1, total: 2 }
    }
    else
    {
        Progress{ done: 2, total: 2 }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Plugin for simulating loading delay when initializing the app.
#[bevy_plugin]
pub(crate) fn LoadingSimPlugin(app: &mut App)
{
    app
        .insert_resource(LoadingStartTime(Duration::default()))
        .add_systems(Update, initialization_timer.track_progress()
            .in_set(ClientSet::InitCore)
            .in_set(ClientFwLoadingSet)
        )
        .add_systems(Update,
            reset_loading_time_start
                .in_set(ClientSet::InitCore)
                .before(initialization_timer)
                .run_if(bevy_replicon::client_just_connected())
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
