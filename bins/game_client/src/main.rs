//! Independent client binary. Can be used to launch games directly from another binary without an intermediating
//! user client.

use std::time::Duration;

use bevy_girk_client_instance::*;
use bevy_girk_utils::*;
use clap::Parser;
use wiring_client_instance::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
struct GameClientCli
{
    #[arg(short = 'T')]
    token: ServerConnectToken,
    #[arg(short = 'S')]
    start_info: GameStartInfo,
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
        .add_directive("bevy=warn".parse().unwrap())
        .add_directive("bevy_app=warn".parse().unwrap())
        .add_directive("bevy_core=warn".parse().unwrap())
        .add_directive("bevy_winit=warn".parse().unwrap())
        .add_directive("bevy_render=warn".parse().unwrap())
        .add_directive("blocking=warn".parse().unwrap())
        .add_directive("naga_oil=warn".parse().unwrap())
        .add_directive("bevy_replicon=info".parse().unwrap());
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("game client started");

    // cli
    let args = GameClientCli::parse();
    let mut token = Some(args.token);
    let mut start_info = Some(args.start_info);

    // make client factory
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();
    let factory = ClickClientFactory { protocol_id, resend_time: Duration::from_millis(100) };

    App::new()
        .add_plugins(ClientInstancePlugin::new(factory, None))
        .add_systems(OnEnter(LoadState::Done), move |mut c: Commands| {
            let token = token.take().unwrap();
            let start_info = start_info.take().unwrap();
            c.queue(ClientInstanceCommand::Start(token, start_info));
        })
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
