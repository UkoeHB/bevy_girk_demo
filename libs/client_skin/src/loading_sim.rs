use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_client_fw::*;
use iyes_progress::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
struct LoadingStartTime(Duration);

//-------------------------------------------------------------------------------------------------------------------

fn setup_timer(mut c: Commands, time: Res<Time>)
{
    c.insert_resource(LoadingStartTime(time.elapsed()));
}

//-------------------------------------------------------------------------------------------------------------------

/// Hacky timer for delaying initialization.
fn initialization_timer(time: Res<Time>, start: Res<LoadingStartTime>) -> Progress
{
    if time.elapsed() < start.0 + Duration::from_millis(500) {
        Progress { done: 0, total: 2 }
    } else if time.elapsed() < start.0 + Duration::from_millis(1000) {
        Progress { done: 1, total: 2 }
    } else {
        Progress { done: 2, total: 2 }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Plugin for simulating loading delay when initializing a game.
pub(super) struct LoadingSimPlugin;

impl Plugin for LoadingSimPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(ClientAppState::Game), setup_timer)
            .add_systems(
                Update,
                initialization_timer
                    .track_progress::<ClientInitState>()
                    .run_if(client_is_initializing)
                    .in_set(ClientFwLoadingSet),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
