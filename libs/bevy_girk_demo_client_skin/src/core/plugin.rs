//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::window::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_client_fw::*;
use iyes_progress::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

//hacky timer for delaying initialization
fn initialization_timer(time: Res<Time>) -> Progress
{
    if time.elapsed_seconds() < 1.0
    {
        Progress{ done: 0, total: 2 }
    }
    else if time.elapsed_seconds() < 2.0
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

/// Initialize the bevy engine.
#[bevy_plugin]
fn BevyEnginePlugin(app: &mut App)
{
    // prepare bevy plugins
    let bevy_plugins = bevy::DefaultPlugins
        .set(
            WindowPlugin{
                primary_window: Some(Window{
                    title        : "BEVY_GIRK: CLICK DEMO".into(),
                    window_theme : Some(WindowTheme::Dark),
                    ..Default::default()
                }),
                ..Default::default()
            }
        )
        .build();

    // reduce input lag on native targets
    //todo: remove this if perf becomes an issue
    #[cfg(not(target_family = "wasm"))]
    let bevy_plugins = bevy_plugins.disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>();

    // use custom logging
    let bevy_plugins = bevy_plugins.disable::<bevy::log::LogPlugin>();

    // add to app
    app.add_plugins(bevy_plugins);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Plugin for setting up a click demo game client skin.
///
/// Prerequisites:
/// - Use `make_game_client_core()` to set up a client app.
#[bevy_plugin]
pub fn ClickClientSkinPlugin(app: &mut App)
{
    app
        .add_plugins(bevy_kot::prelude::ReactPlugin)
        .add_plugins(BevyEnginePlugin)
        .add_plugins(UiPlugin)

        //temp: add initialization delay
        .add_systems(Update, initialization_timer.track_progress()
            .in_set(ClientSet::InitStartup)
            .in_set(ClientFWLoadingSet))
        ;
}

//-------------------------------------------------------------------------------------------------------------------
