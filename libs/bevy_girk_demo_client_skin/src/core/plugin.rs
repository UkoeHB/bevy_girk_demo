//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::window::*;
use bevy_fn_plugin::bevy_plugin;

//standard shortcuts


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
        .add_plugins(UiPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
