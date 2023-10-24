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
    app.add_plugins(
            bevy::DefaultPlugins
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
                .build().disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>()
        )
        .insert_resource(bevy::winit::WinitSettings{
            focused_mode   : UpdateMode::Reactive{ max_wait: std::time::Duration::from_millis(100) },
            unfocused_mode : UpdateMode::ReactiveLowPower{ max_wait: std::time::Duration::from_secs(1) },
            ..Default::default()
        });
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Plugin for setting up a click demo user client.
///
/// Prerequisites:
/// - must add a bevy_girk `HostUserClient` to the app
#[bevy_plugin]
pub fn ClickUserClientPlugin(app: &mut App)
{
    app
        .add_plugins(BevyEnginePlugin)
        .add_plugins(HostClientPlugin)
        .add_plugins(UIPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
