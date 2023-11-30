//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

//todo: pass by value on construction
const _FRAME_RATE: f64 = 100.0;
const WINDOWED_TOGGLE_KEY: KeyCode = KeyCode::W;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Initialize the bevy engine.
#[bevy_plugin]
fn BevyEnginePlugin(app: &mut App)
{
    app.add_plugins(bevy::DefaultPlugins);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Setup bevy_framepace
fn framepace_setup(mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>)
{
    //framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(FRAME_RATE);
    framepace_settings.limiter = bevy_framepace::Limiter::Auto;
}

/// Initialize third-party dependencies
#[bevy_plugin]
fn ThirdPartyPlugin(app: &mut App)
{
    app.add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, framepace_setup);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// System for toggling the window mode.
fn toggle_window_mode(mut window: Query<&mut Window, With<bevy::window::PrimaryWindow>>)
{
    let window: &mut Window = &mut window.single_mut();
    if window.mode == bevy::window::WindowMode::Windowed
        { window.mode = bevy::window::WindowMode::SizedFullscreen; }
    else
        { window.mode = bevy::window::WindowMode::Windowed; }
}

/// System for initializing the vsync setting.
fn set_vsync(mut window: Query<&mut Window, With<bevy::window::PrimaryWindow>>)
{
    window.single_mut().present_mode = bevy::window::PresentMode::AutoVsync;
}

/// Plugin for systems related to client configuration.
#[bevy_plugin]
fn ConfigPlugin(app: &mut App)
{
    app
        .add_systems(Startup, set_vsync)
        .add_systems(Update, toggle_window_mode
            .run_if(|key_pressed: Res<Input<KeyCode>>| key_pressed.just_pressed(WINDOWED_TOGGLE_KEY)));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Initialize the GIRK components.
#[derive(Component)]
struct FPSIndicator;

fn bevy_girk_setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    // prepare 2D camera
    commands.spawn(Camera2dBundle::default());

    // add a basic sprite
    commands.spawn(
            SpriteBundle{
                    texture   : asset_server.load("logo.png"),
                    transform : Transform::from_xyz(-400., 250., 0.),
                    ..default()
                }
        );

    // add FPS indicator
    let text_style = TextStyle{
            font      : asset_server.load("fonts/FiraSans-Bold.ttf").clone(),
            font_size : 45.0,
            color     : Color::WHITE,
        };
    let fps_text =
        Text2dBundle{
                text : Text::from_sections(vec![
                        TextSection{
                                value : "FPS: ".to_string(),
                                style : text_style.clone(),
                            },
                        TextSection{
                                value : "0".to_string(),  //for the FPS value
                                style : text_style.clone(),
                            }
                    ]).with_alignment(TextAlignment::Left),
                ..default()
            };

    commands.spawn( (FPSIndicator, fps_text) );
}

#[bevy_plugin]
fn GIRKSetupPlugin(app: &mut App)
{
    app
        .add_plugins(FPSTrackerPlugin)
        .add_plugins(ConfigPlugin)
        .add_systems(Startup, bevy_girk_setup)
        .add_systems(Update, refresh_fps_indicator.after(FPSTrackerSet));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Realtime systems
fn refresh_fps_indicator(mut indicator_query: Query<&mut Text, With<FPSIndicator>>, fps_tracker: Res<FPSTracker>)
{
    // 1. only refresh once per second
    if fps_tracker.current_time().as_secs() <= fps_tracker.previous_time().as_secs() { return }

    // 2. refresh
    let indicator_value = &mut indicator_query.single_mut().sections[1].value;
    *indicator_value = format!("{}", fps_tracker.fps());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub fn GIRKClientPluginTemp(app: &mut App)
{
    app
        .add_plugins(BevyEnginePlugin)
        .add_plugins(ThirdPartyPlugin)
        .add_plugins(GIRKSetupPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
