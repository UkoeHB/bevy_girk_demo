/*
impl notes

- ClientStarter (reactive resource): bevy_girk
    - tracks GameStartInfo for currently-active game in case it needs to be combined with a new connect token
    to reconnect
    - when set, will trigger popup to block screen while game reconnecting in ClientAppState::Client
- ClientInstanceCommand
    x- ::Start -> sent by ClientStarter::start
    - ::StartLocal(game launch pack) -> must be sent to start a local game
        - reject if ClientStarter is set or if a local game is already running
    x- ::RequestConnectToken -> sent by bevy_girk, turns into ClientInstanceReport::RequestConnectToken
    x- ::End -> send when entered ClientState::GameOver
    x- ::Abort -> send if host server sends a game abort message or if local game reports an abort
- ClientInstanceReport: bevy event
    x- ::RequestConnectToken(game_id)
        x- check if game_id matches ClientStarter
        x- send UserToHostRequest::GetConnectToken
            - save pending request to log results properly (??)
            - response = HostToUserResponse::ConnectToken
                - insert connect token (system will handle starting the game)
        x- log
    x- ::Ended(game_id)
        - clear ClientStarter if game_id matches
        - log
    x- ::Aborted(u64),
        - clear ClientStarter if game_id matches
        - log
- HostToUserMsg: simplenet channel message
    x- ::GameStart
        x- if game id doesn't match ClientStarter and game is currently running, abort the current game
        x- set ClientStarter, cache connect token
        x- use system with run_if(in_state(ClientAppState::Client)) and in_state(LoadState::Done) to start the
        game if client starter is set and a connect token is cached
            x- need to delay starting the new game until back in ClientAppState::Client, if game currently
            running (e.g. local-player game)
    x- ::GameAborted
        x- clear ClientStarter if game_id matches
        x- send ClientInstanceCommand::Abort
    x- ::GameOver
        x- clear ClientStarter if game_id matches
        x- broadcast game over report
- host to user client
    x- on death, need to reconstruct the client on a timer (0.5s)
        x- this loop may occur if the server is at max capacity; it lets us poll until a connection is allowed
- LocalGameManager (resource): bevy_girk
    x- OnEnter(ClientAppState::Client) -> manager.take_report()
        x- log
*/

use bevy::prelude::*;
use bevy::render::render_resource::{CachedPipelineState, PipelineCache};
use bevy::render::{MainWorld, RenderApp};
use bevy::window::*;
use bevy::winit::UpdateMode;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::ClientAppState;
use iyes_progress::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

/// A `Resource` in the main world that stores the number of pipelines that are ready and total number of
/// pipelines.
#[derive(Resource, Default, Debug)]
struct PipelinesReady(usize, usize);

//-------------------------------------------------------------------------------------------------------------------

fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, cache: Res<PipelineCache>)
{
    let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() else { return };
    let num_pipelines = cache
        .pipelines()
        .filter(|pipeline| !matches!(pipeline.state, CachedPipelineState::Err(_)))
        .count();
    let count = cache
        .pipelines()
        .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Ok(_)))
        .count();

    pipelines_ready.0 = count;
    pipelines_ready.1 = num_pipelines;
}

//-------------------------------------------------------------------------------------------------------------------

/// Initialize the bevy engine.
struct BevyEnginePlugin;

impl Plugin for BevyEnginePlugin
{
    fn build(&self, app: &mut App)
    {
        // prepare bevy plugins
        #[allow(unused_mut)]
        let mut default_plugins = bevy::DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BEVY_GIRK: CLICK DEMO".into(),
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        });
        #[cfg(target_family = "wasm")]
        {
            default_plugins =
                default_plugins.set(AssetPlugin { meta_check: bevy::asset::AssetMetaCheck::Never, ..default() });
        }
        let bevy_plugins = default_plugins.build();

        // reduce input lag on native targets
        #[cfg(not(target_family = "wasm"))]
        let bevy_plugins = bevy_plugins.disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>();

        // use custom logging
        let bevy_plugins = bevy_plugins.disable::<bevy::log::LogPlugin>();

        // time plugin added separately by bevy_girk
        let bevy_plugins = if app.is_plugin_added::<bevy::time::TimePlugin>() {
            bevy_plugins.disable::<bevy::time::TimePlugin>()
        } else {
            bevy_plugins
        };
        let bevy_plugins = if app.is_plugin_added::<bevy::state::app::StatesPlugin>() {
            bevy_plugins.disable::<bevy::state::app::StatesPlugin>()
        } else {
            bevy_plugins
        };

        // add to app
        app.add_plugins(bevy_plugins)
            .insert_resource(bevy::winit::WinitSettings {
                focused_mode: UpdateMode::reactive(std::time::Duration::from_millis(10)),
                unfocused_mode: UpdateMode::reactive_low_power(std::time::Duration::from_millis(200)),
                ..Default::default()
            });
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn loadstate_progress(state: Res<State<LoadState>>, progress: Res<LoadProgress>) -> Progress
{
    let state = match state.get() {
        LoadState::Loading => 0,
        LoadState::Done => 1,
    };
    let (pending, total) = progress.loading_progress();

    Progress {
        done: state + total.saturating_sub(pending) as u32,
        total: 1 + total as u32,
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn pipeline_progress(pipelines: Res<PipelinesReady>) -> Progress
{
    Progress { done: pipelines.0 as u32, total: pipelines.1 as u32 }
}

//-------------------------------------------------------------------------------------------------------------------

fn setup(mut c: Commands)
{
    c.spawn(Camera2d);
}

//-------------------------------------------------------------------------------------------------------------------

/// Plugin for setting up a click demo user client.
///
/// Prerequisites:
/// - `ClientInstancePlugin` plugin *with* game factory for local games
/// - [`TimerConfigs`] resource
/// - [`HostClientConstructor`] resource
pub struct ClickClientGlobalPlugin;

impl Plugin for ClickClientGlobalPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_resource::<PipelinesReady>()
            .add_plugins(BevyEnginePlugin)
            .add_plugins(CobwebUiPlugin)
            .load("ui_common/constants.cob")
            .add_systems(PreStartup, setup)
            .add_systems(
                Update,
                (
                    loadstate_progress.track_progress::<ClientAppState>(),
                    pipeline_progress.track_progress::<ClientAppState>(),
                )
                    .run_if(in_state(ClientAppState::Loading)),
            );
        app.sub_app_mut(RenderApp)
            .add_systems(ExtractSchedule, update_pipelines_ready);
    }
}

//-------------------------------------------------------------------------------------------------------------------
