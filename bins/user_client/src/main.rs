//module tree

//local shortcuts
use bevy_girk_demo_user_client::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use clap::Parser;

//standard shortcuts
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
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn get_systime_millis() -> u128
{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()
}

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

    // cli args
    let args = ClientCli::parse();

    // define client id
    let client_id = args.client_id.unwrap_or_else(get_systime_millis);

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

    // define configs (TEMPORARY: use asset instead ?)
    let timer_configs = TimerConfigs{
            ack_request_timeout_ms      : ACK_TIMEOUT_MILLIS + 1_000,
            ack_request_timer_buffer_ms : 4_000,
            lobby_list_refresh_ms       : 10_000,
        };

    // build and launch the bevy app
    let mut app = App::new();
    app.insert_resource(client)
        .insert_resource(timer_configs)
        .add_plugins(ClickUserClientPlugin)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
