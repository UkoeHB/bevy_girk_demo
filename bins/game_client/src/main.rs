//! Independent client binary. Can be used to launch games directly from another binary without an intermediating
//! user client.

use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_client_fw::ClientAppState;
use bevy_girk_client_instance::*;
use bevy_girk_game_instance::GameStartInfo;
use bevy_girk_utils::*;
use bevy_girk_wiring_common::ServerConnectToken;
use clap::Parser;
use wiring_client_instance::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
struct GameClientCli
{
    /// ServerConnectToken
    #[arg(short = 'T', value_parser = parse_json::<ServerConnectToken>)]
    token: Option<ServerConnectToken>,
    /// GameStartInfo
    #[arg(short = 'S', value_parser = parse_json::<GameStartInfo>)]
    start_info: Option<GameStartInfo>,
}

//-------------------------------------------------------------------------------------------------------------------

fn start_game_system(
    mut token: Option<ServerConnectToken>,
    mut start_info: Option<GameStartInfo>,
) -> impl IntoSystem<(), (), ()>
{
    IntoSystem::into_system(move |mut c: Commands| {
        let Some(token) = token.take() else { return };
        let Some(start_info) = start_info.take() else { return };
        c.queue(ClientInstanceCommand::Start(token, start_info));
    })
}

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // log to stderr (not stdout, which is piped to the parent process for sending game instance reports)
    //todo: log to file instead (use CLI configs?)
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::TRACE.into())
        .from_env()
        .unwrap()
        .add_directive("bevy=trace".parse().unwrap())
        .add_directive("bevy_cobweb=info".parse().unwrap())
        .add_directive("bevy_cobweb_ui=info".parse().unwrap())
        .add_directive("bevy_app=warn".parse().unwrap())
        .add_directive("bevy_core=warn".parse().unwrap())
        .add_directive("bevy_winit=warn".parse().unwrap())
        .add_directive("bevy_render=warn".parse().unwrap())
        .add_directive("blocking=warn".parse().unwrap())
        .add_directive("naga_oil=warn".parse().unwrap())
        .add_directive("bevy_replicon=info".parse().unwrap())
        .add_directive("winit=warn".parse().unwrap());
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("game client started");

    // cli
    let args = GameClientCli::parse();
    let token: Option<ServerConnectToken> = args.token;
    let start_info: Option<GameStartInfo> = args.start_info;

    // make client factory
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();
    let factory = ClickClientFactory { protocol_id, resend_time: Duration::from_millis(100) };

    let mut app = App::new();
    app.add_plugins(ClientInstancePlugin::new(factory, None))
        // Can't do this in OnEnter because it internally forces a state transition. State transitions can't be
        // executed recursively.
        // Can't do this in Update otherwise we might skip past ClientInitState::InProgress.
        .add_systems(
            PreUpdate,
            start_game_system(token, start_info).run_if(in_state(ClientAppState::Client)),
        )
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
