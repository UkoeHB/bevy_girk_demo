use std::str::FromStr;
use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_client_instance::*;
use bevy_girk_game_instance::GameFactory;
use bevy_girk_utils::Rand64;
use clap::Parser;
use user_client::*;
use wasm_timer::{SystemTime, UNIX_EPOCH};
use wiring_backend::*;
use wiring_client_instance::ClickClientFactory;
use wiring_game_instance::ClickGameFactory;

//-------------------------------------------------------------------------------------------------------------------

//todo: specify app data file path (e.g. contains auth keys [temp solution before 'login'-style auth], logs,
// settings)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClientCli
{
    /// Specify the client id (will be random if unspecified).
    #[arg(long = "id")]
    client_id: Option<u128>,
    /// Alt: GIRK_HOST_ADDR env variable
    #[arg(long = "addr")]
    server_addr: Option<String>,
    /// Alt: GIRK_HOST_IS_WSS env variable
    #[arg(long)]
    host_is_wss: Option<bool>,
}

//-------------------------------------------------------------------------------------------------------------------

fn get_systime_millis() -> u128
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // setup wasm tracing
    #[cfg(target_family = "wasm")]
    {
        console_error_panic_hook::set_once();
        //tracing_wasm::set_as_global_default();
    }

    // log to stdout
    //todo: log to file?
    #[cfg(not(target_family = "wasm"))]
    {
        let filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
            .from_env()
            .unwrap()
            .add_directive("ezsockets=debug".parse().unwrap())
            .add_directive("bevy_girk_game_instance=trace".parse().unwrap())
            .add_directive("client_core=trace".parse().unwrap())
            .add_directive("user_client=trace".parse().unwrap())
            .add_directive("bevy_girk_wiring=trace".parse().unwrap())
            .add_directive("bevy_girk_utils=trace".parse().unwrap())
            .add_directive("bevy_simplenet=debug".parse().unwrap())
            .add_directive("renet2=info".parse().unwrap())
            .add_directive("renet2_netcode=info".parse().unwrap())
            .add_directive("renetcode2=info".parse().unwrap());
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(filter)
            .with_writer(std::io::stdout)
            .init();
    }

    // cli args
    let args = ClientCli::parse();
    tracing::trace!(?args);

    // unwrap args
    let client_id = args.client_id.unwrap_or_else(get_systime_millis);
    let server_addr = args
        .server_addr
        .or_else(|| std::option_env!("GIRK_HOST_ADDR").map(|s| s.into()))
        .unwrap_or_else(|| "127.0.0.1:48888".into());
    let host_is_wss = args
        .host_is_wss
        .or_else(|| std::option_env!("GIRK_HOST_IS_WSS").map(|s| bool::from_str(s).unwrap_or_default()))
        .unwrap_or_default();

    // set asset directory location
    #[cfg(not(target_family = "wasm"))]
    {
        if let Err(err) = bevy_girk_utils::try_set_bevy_asset_root(2) {
            panic!("Could not set bevy asset root: {}", err.to_string());
        }
    }

    //TODO: use hasher directly?
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // make URL
    let host = if host_is_wss { "wss" } else { "ws" };
    let url = format!("{host}://{}/ws", server_addr.as_str());
    tracing::info!("connecting to host server: {}", url.as_str());

    // prep to launch client
    // - todo: receive URL from HTTP(s) server, and load the HTTP(s) URL from an asset
    let make_client = move || {
        host_user_client_factory().new_client(
            enfync::builtin::Handle::default(), // automatically selects native/WASM runtime
            // TODO: use auth server to get this?
            url::Url::parse(url.as_str()).unwrap(),
            // TODO: use auth tokens and an auth server
            bevy_simplenet::AuthRequest::None { client_id },
            bevy_simplenet::ClientConfig::default(),
            // auto-detects connection type for games (udp/webtransport/websockets)
            HostUserConnectMsg::new(),
        )
    };

    // timer configs for the user client (TEMPORARY: use asset instead ?)
    let timer_configs = TimerConfigs {
        host_reconstruct_loop_ms: 500,
        token_request_loop_ms: 500,
        ack_request_timeout_ms: ACK_TIMEOUT_MILLIS + 1_000,
        ack_request_timer_buffer_ms: 4_000,
        lobby_list_refresh_ms: 10_000,
    };

    // factory for local-player games
    let game_factory = GameFactory::new(ClickGameFactory);

    // client factory for setting up games
    let factory = ClickClientFactory { protocol_id, resend_time: Duration::from_millis(100) };

    // build and launch the bevy app
    App::new()
        .add_plugins(ClientInstancePlugin::new(factory, Some(game_factory)))
        .insert_resource(HostClientConstructor::new(make_client))
        .insert_resource(timer_configs)
        .add_plugins(ClickUserClientPlugin)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
