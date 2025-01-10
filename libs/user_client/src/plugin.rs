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
    - ::RequestConnectToken(game_id)
        - check if game_id matches ClientStarter
        - send UserToHostRequest::GetConnectToken
            - save pending request to log results properly (??)
            - response = HostToUserResponse::ConnectToken
                - insert connect token (system will handle starting the game)
        - log
    - ::Ended(game_id)
        - clear ClientStarter if game_id matches
        - log
    - ::Aborted(u64),
        - clear ClientStarter if game_id matches
        - log
- HostToUserMsg: simplenet channel message
    - ::GameStart
        - if game id doesn't match ClientStarter and game is currently running, abort the current game
        - set ClientStarter and insert connect token
        - use system with run_if(in_state(ClientAppState::Client)) and in_state(LoadState::Done) to start the
        game if client starter is set and a connect token is inserted
            - need to delay starting the new game until back in ClientAppState::Client, if game currently
            running (e.g. local-player game)
    - ::GameAborted
        - clear ClientStarter if game_id matches
        - send ClientInstanceCommand::Abort
    - ::GameOver
        - clear ClientStarter if game_id matches
        - send ClientInstanceCommand::End
x- host to user client
    - on death, need to reconstruct the client on a timer (0.5s)
        - this loop may occur if the server is at max capacity; it lets us poll until a connection is allowed
- LocalGameManager (resource): bevy_girk
    - OnEnter(ClientAppState::Client) -> manager.take_report()
        - log
*/

use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::UpdateMode;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Initialize the bevy engine.
struct BevyEnginePlugin;

impl Plugin for BevyEnginePlugin
{
    fn build(&self, app: &mut App)
    {
        // prepare bevy plugins
        let bevy_plugins = bevy::DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "BEVY_GIRK: CLICK DEMO".into(),
                    window_theme: Some(WindowTheme::Dark),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .build();

        // reduce input lag on native targets
        #[cfg(not(target_family = "wasm"))]
        let bevy_plugins = bevy_plugins.disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>();

        // use custom logging
        let bevy_plugins = bevy_plugins.disable::<bevy::log::LogPlugin>();

        // add to app
        app.add_plugins(bevy_plugins)
            .insert_resource(bevy::winit::WinitSettings {
                focused_mode: UpdateMode::Reactive { wait: std::time::Duration::from_millis(10) },
                unfocused_mode: UpdateMode::ReactiveLowPower { wait: std::time::Duration::from_millis(250) },
                ..Default::default()
            });
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Plugin for setting up a click demo user client.
///
/// Prerequisites:
/// - You must add a bevy_girk `HostUserClient` to the app.
/// - You must add a [`TimeoutConfigs`] resource to the app.
/// - You must add a [`ClientLaunchConfigs`] resource to the app.
/// - You must add a [`HostClientConstructor`] resource to the app.
pub struct ClickUserClientPlugin;

impl Plugin for ClickUserClientPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            // Dependencies
            .add_plugins(BevyEnginePlugin)
            .add_plugins(ReactPlugin)
            .add_plugins(UserClientUtilsPlugin)
            // Crate plugins
            .add_plugins(HostClientPlugin)
            .add_plugins(LobbiesPlugin)
            .add_plugins(GamePlugin)
            .add_plugins(UiPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
