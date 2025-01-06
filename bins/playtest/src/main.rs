use std::net::Ipv6Addr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy_girk_client_instance::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use clap::Parser;
use enfync::{AdoptOrDefault, Handle};
use game_core::*;
use wiring_backend::*;
use wiring_game_instance::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
struct PlaytestCli
{
    /// Specify the number of clients (defaults to 1, minimum is 1).
    #[arg(long)]
    clients: Option<usize>,
    /// Specify the location of the game instance binary (will use the debug build directory by default).
    game: Option<String>,
    /// Specify the location of the game client binary (will use the debug build directory by default).
    client: Option<String>,
}

//-------------------------------------------------------------------------------------------------------------------

const GAME_INSTANCE_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_instance");
const GAME_CLIENT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_client");

//-------------------------------------------------------------------------------------------------------------------

fn get_systime() -> Duration
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
}

//-------------------------------------------------------------------------------------------------------------------

//todo: move this somewhere else...
fn make_click_game_configs() -> ClickGameFactoryConfig
{
    // game duration
    let game_ticks_per_sec = 20;
    let game_num_ticks = game_ticks_per_sec * 30;

    // versioning
    //todo: use hasher directly?
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks = game_ticks_per_sec * 5;
    let game_prep_ticks = 0;
    let game_over_ticks = game_ticks_per_sec * 3;

    // server setup config
    let server_setup_config = GameServerSetupConfig {
        protocol_id,
        expire_secs: 10u64,
        timeout_secs: 5i32,
        server_ip: Ipv6Addr::LOCALHOST,
    };

    // game framework config
    let game_fw_config = GameFwConfig::new(game_ticks_per_sec, max_init_ticks, game_over_ticks);

    // game duration config
    let game_duration_config = GameDurationConfig::new(game_prep_ticks, game_num_ticks);

    // click game factory config
    let game_factory_config = ClickGameFactoryConfig {
        server_setup_config,
        game_fw_config,
        game_duration_config,
        resend_time: Duration::from_millis(300),
    };

    game_factory_config
}

//-------------------------------------------------------------------------------------------------------------------

fn run_playtest(
    num_clients: usize,
    launch_pack: GameLaunchPack,
    game_instance_path: String,
    game_client_path: String,
)
{
    // launch in task
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let spawner_clone = spawner.clone();
    let task = spawner.spawn(async move {
        // launch game
        tracing::trace!("launching game instance for playtest");
        let (game_report_sender, mut game_report_receiver) = new_io_channel::<GameInstanceReport>();
        let game_launcher = GameInstanceLauncherProcess::new(game_instance_path, spawner_clone.clone());
        let mut game_instance = game_launcher.launch(launch_pack, game_report_sender);

        // wait for game start report
        let Some(GameInstanceReport::GameStart(_, mut report)) = game_report_receiver.recv().await else {
            tracing::error!("failed getting game start report for playtest");
            return;
        };

        // prepare to launch the clients
        let Some(meta) = &report.metas.native_meta else {
            tracing::error!("missing native meta for setting up playtest renet client");
            return;
        };

        // launch game clients
        let mut client_instances = Vec::default();
        for _ in 0..num_clients {
            let Ok(token) = new_connect_token_native(meta, get_systime(), start_info.client_id) else {
                tracing::error!("failed producing connect token for playtest");
                continue;
            };

            let Some(start_info) = report.start_infos.pop() else {
                tracing::error!("missing start info for playtest");
                continue;
            };

            let Ok(token_ser) = serde_json::to_string(&token) else {
                tracing::error!(game_id, "failed serializing server connect token for playtest game client");
                continue;
            };

            let Ok(start_info_ser) = serde_json::to_string(&start_info) else {
                tracing::error!(game_id, "failed serializing game start info for playtest game client");
                continue;
            };

            tracing::trace!(start_info.client_id, "launching game client for playtest");

            let Ok(child_process) = tokio::process::Command::new(&game_client_path)
                .args(["-T", &token_ser, "-S", &start_info_ser])
                .spawn()
            else {
                tracing::error!("failed launching game client for playtest at {:?}", game_client_path);
                continue;
            };

            client_instances.push(child_process);
        }

        // wait for clients to close
        // - we must wait for client closure to avoid zombie process leak
        for mut client_instance in client_instances {
            if client_instance.wait().await.is_err() {
                tracing::warn!("playtest client instance closed with error");
                client_instance.kill().await;
            }
        }

        // command game instance to abort
        // - we assume if the clients are closed then the game should die
        // - this will do nothing if the game instance already closed
        let _ = game_instance.send_command(GameInstanceCommand::Abort);

        // wait for game instance to close
        if !game_instance.get().await {
            tracing::warn!("playtest instance closed with error");
        }

        // get game instance report
        let Some(GameInstanceReport::GameOver(_, _game_over_report)) = game_report_receiver.recv().await else {
            tracing::error!("did not receive game over report for playtest");
            return;
        };
    });

    let _ = enfync::blocking::extract(task);
}

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // logging
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::TRACE.into())
        .from_env()
        .unwrap();
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    // set asset directory location
    #[cfg(not(target_family = "wasm"))]
    {
        if let Err(err) = bevy_girk_utils::try_set_bevy_asset_root(2) {
            panic!("Could not set bevy asset root: {}", err.to_string());
        }
    }

    // env
    let args = PlaytestCli::parse();
    tracing::trace!(?args);

    // unwrap args
    let num_clients = args.clients.unwrap_or(1usize).max(1usize);
    let game_instance_path = args
        .game
        .unwrap_or_else(|| String::from(GAME_INSTANCE_PATH));
    let game_client_path = args
        .client
        .unwrap_or_else(|| String::from(GAME_CLIENT_PATH));

    // lobby contents
    let mut players = Vec::default();
    for idx in 0..num_clients {
        players.push((ConnectionType::Native, idx as u128));
    }

    let lobby_contents = ClickLobbyContents {
        id: 0u64,
        owner_id: 0u128,
        config: ClickLobbyConfig { max_players: num_clients as u16, max_watchers: 0u16 },
        players,
        watchers: Vec::default(),
    };

    // launch pack
    let game_configs = make_click_game_configs();
    let Ok(launch_pack) = get_launch_pack(game_configs, lobby_contents) else {
        tracing::error!("failed getting launch pack for playtest");
        return;
    };

    // run it
    run_playtest(num_clients, launch_pack, game_instance_path, game_client_path);
}

//-------------------------------------------------------------------------------------------------------------------
