//local shortcuts
use bevy_girk_demo_client_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_client_fw::*;
use bevy_girk_game_fw::*;
use iyes_progress::*;

//standard shortcuts
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn reset_init_progress(mut progress: Query<&mut GameInitProgress>)
{
    progress.single_mut().0 = 0.0;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
struct LoadingTimeStart(Duration);

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn reset_loading_time_start(time: Res<Time>, mut start: ResMut<LoadingTimeStart>)
{
    start.0 = time.elapsed();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Hacky timer for delaying initialization.
fn initialization_timer(time: Res<Time>, start: Res<LoadingTimeStart>) -> Progress
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
        .insert_resource(LoadingTimeStart(Duration::default()))
        .add_systems(Update, initialization_timer.track_progress()
            .in_set(ClientSet::InitCore)
            .in_set(ClientFWLoadingSet)
        )
        .add_systems(Update,
            reset_init_progress
                .run_if(bevy_replicon::client_just_disconnected())
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
