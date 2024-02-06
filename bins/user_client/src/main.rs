//module tree

//local shortcuts
use bevy_girk_demo_user_client::*;
use bevy_girk_demo_wiring_backend::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_client_instance::*;
use bevy_girk_user_client_utils::*;
use clap::Parser;
use enfync::AdoptOrDefault;

//standard shortcuts
use std::sync::Arc;
use wasm_timer::{SystemTime, UNIX_EPOCH};

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

//todo: specify app data file path (e.g. contains auth keys [temp solution before 'login'-style auth], logs, settings)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClientCli
{
    /// Specify the client id (will be random if unspecified).
    #[arg(long = "id")]
    client_id: Option<u128>,
    /// Specify the location of the game instance binary (will use the debug build directory by default).
    game: Option<String>,
    /// Specify the location of the game client binary (will use the debug build directory by default).
    client: Option<String>,
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn get_systime_millis() -> u128
{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

const GAME_INSTANCE_PATH : &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_instance");
const GAME_CLIENT_PATH : &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_client");

//-------------------------------------------------------------------------------------------------------------------
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
        .from_env().unwrap()
        .add_directive("ezsockets=error".parse().unwrap())
        .add_directive("bevy_girk_game_instance=trace".parse().unwrap())
        .add_directive("bevy_girk_demo_user_client=trace".parse().unwrap())
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
    let game_instance_path = args.game.unwrap_or_else(|| String::from(GAME_INSTANCE_PATH));
    let game_client_path = args.client.unwrap_or_else(|| String::from(GAME_CLIENT_PATH));

    // set asset directory location
    #[cfg(not(target_family = "wasm"))]
    {
        if let Err(err) = bevy_girk_utils::try_set_bevy_asset_root(2)
        {
            panic!("Could not set bevy asset root: {}", err.to_string());
        }
    }

    // launch client
    // - todo: pass in URL and client id via CLI
    let client = host_user_client_factory().new_client(
            enfync::builtin::Handle::default(),  //automatically selects native/WASM runtime
            url::Url::parse("ws://127.0.0.1:48888/ws").unwrap(),
            bevy_simplenet::AuthRequest::None{ client_id },
            bevy_simplenet::ClientConfig::default(),
            ()
        );

    // timer configs (TEMPORARY: use asset instead ?)
    let timer_configs = TimerConfigs{
            ack_request_timeout_ms      : ACK_TIMEOUT_MILLIS + 1_000,
            ack_request_timer_buffer_ms : 4_000,
            lobby_list_refresh_ms       : 10_000,
        };

    // launcher configs
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let spawner_fn = Arc::new(move || { spawner.clone() });
    let launcher_configs = ClientLaunchConfigs{
            local: LocalPlayerLauncherConfigNative{
                spawner_fn           : spawner_fn.clone(),
                game_instance_path   : game_instance_path.clone(),
                client_instance_path : game_client_path.clone(),
            },
            multiplayer: MultiPlayerLauncherConfigNative{
                spawner_fn,
                client_instance_path   : game_client_path,
                client_instance_config : ClientInstanceConfig{ reconnect_interval_secs: 5u32 },
            }
        };

    // build and launch the bevy app
    App::new()
        .insert_resource(client)
        .insert_resource(timer_configs)
        .insert_resource(launcher_configs)
        .add_plugins(ClickUserClientPlugin)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
