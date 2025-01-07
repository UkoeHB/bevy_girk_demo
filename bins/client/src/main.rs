use std::sync::Arc;

use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_client_instance::*;
use bevy_girk_user_client_utils::*;
use clap::Parser;
use enfync::AdoptOrDefault;
use user_client::*;
use wasm_timer::{SystemTime, UNIX_EPOCH};
use wiring_backend::*;

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
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::WARN.into())
        .from_env()
        .unwrap()
        .add_directive("ezsockets=error".parse().unwrap())
        .add_directive("bevy_girk_game_instance=trace".parse().unwrap())
        .add_directive("user_client=trace".parse().unwrap())
        .add_directive("user_client=trace".parse().unwrap())
        .add_directive("bevy_girk_wiring=trace".parse().unwrap())
        .add_directive("bevy_girk_utils=trace".parse().unwrap())
        .add_directive("bevy_simplenet=error".parse().unwrap());
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .init();

    // cli args
    let args = ClientCli::parse();
    tracing::trace!(?args);

    // unwrap args
    let client_id = args.client_id.unwrap_or_else(get_systime_millis);

    // set asset directory location
    #[cfg(not(target_family = "wasm"))]
    {
        if let Err(err) = bevy_girk_utils::try_set_bevy_asset_root(2) {
            panic!("Could not set bevy asset root: {}", err.to_string());
        }
    }

    //TODO: use hasher directly?
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // launch client
    // - todo: receive URL from HTTP server, and load the HTTP URL from an asset
    let client = host_user_client_factory().new_client(
        enfync::builtin::Handle::default(), //automatically selects native/WASM runtime
        url::Url::parse("ws://127.0.0.1:48888/ws").unwrap(),
        bevy_simplenet::AuthRequest::None { client_id },
        bevy_simplenet::ClientConfig::default(),
        (),
    );

    // timer configs for the user client (TEMPORARY: use asset instead ?)
    let timer_configs = TimerConfigs {
        ack_request_timeout_ms: ACK_TIMEOUT_MILLIS + 1_000,
        ack_request_timer_buffer_ms: 4_000,
        lobby_list_refresh_ms: 10_000,
    };

    // launcher for local-player games
    let game_factory = GameFactory::new(ClickGameFactory);
    let local_launcher =
        LocalGameLauncher(GameInstanceLauncher::new(GameInstanceLauncherLocal::new(game_factory)));

    // client factory for setting up games
    let mut factory =
        ClientFactory::new(ClickClientFactory { protocol_id, resend_time: Duration::from_millis(100) });

    // build and launch the bevy app
    let mut app = App::new();
    factory.add_plugins(&mut app);
    app.insert_resource(factory);

    app.insert_resource(client)
        .insert_resource(timer_configs)
        .insert_resource(local_launcher)
        .add_plugins(UserClientPlugin)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
