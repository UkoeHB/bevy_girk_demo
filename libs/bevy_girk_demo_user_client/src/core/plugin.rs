//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::UpdateMode;
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
    #[cfg(not(target_family = "wasm"))]
    let bevy_plugins = bevy_plugins.disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>();

    // use custom logging
    let bevy_plugins = bevy_plugins.disable::<bevy::log::LogPlugin>();

    // add to app
    app.add_plugins(bevy_plugins)
        .insert_resource(bevy::winit::WinitSettings{
            focused_mode   : UpdateMode::Reactive{ wait: std::time::Duration::from_millis(100) },
            unfocused_mode : UpdateMode::ReactiveLowPower{ wait: std::time::Duration::from_millis(250) },
            ..Default::default()
        });
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Plugin for setting up a click demo user client.
///
/// Prerequisites:
/// - You must add a bevy_girk `HostUserClient` to the app.
/// - You must add a `TimeoutConfigs` resource to the app.
#[bevy_plugin]
pub fn ClickUserClientPlugin(app: &mut App)
{
    app
        .add_plugins(bevy_kot::prelude::ReactPlugin)
        .add_plugins(BevyEnginePlugin)
        .add_plugins(HostClientPlugin)
        .add_plugins(LobbiesPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(UiPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
