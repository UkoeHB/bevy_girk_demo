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
    app.add_plugins(
            bevy::DefaultPlugins.set(
                WindowPlugin{
                    primary_window: Some(Window{
                        title        : "BEVY_GIRK: CLICK DEMO".into(),
                        window_theme : Some(WindowTheme::Dark),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            )
        )
        .insert_resource(bevy::winit::WinitSettings::desktop_app());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Setup bevy_framepace
fn framepace_setup(mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>)
{
    framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(FRAME_RATE);
    //framepace_settings.limiter = bevy_framepace::Limiter::Auto;
    //framepace_settings.limiter = bevy_framepace::Limiter::Off;
}

/// Initialize framepace plugin
#[bevy_plugin]
fn FramepacePlugin(app: &mut App)
{
    app.add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, framepace_setup);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub fn ClickUserClientPlugin(app: &mut App)
{
    app
        .add_plugins(BevyEnginePlugin)
        .add_plugins(FramepacePlugin)
        .add_plugins(UIPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
