//module tree

//local shortcuts
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_wiring_backend::*;
use bevy_girk_demo_wiring_game_instance::*;

//third-party shortcuts
use bevy_girk_client_instance::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_kot_utils::*;
use clap::Parser;
use enfync::{AdoptOrDefault, Handle};

//standard shortcuts
use std::net::Ipv6Addr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
struct PlaytestCli
{
    #[arg(long)]
    clients: Option<usize>,
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

//todo: inject these
const GAME_INSTANCE_PATH : &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_instance");
const GAME_CLIENT_PATH : &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_client");

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn get_systime() -> Duration
{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default()
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

//todo: move this somewhere else...
fn make_click_game_configs() -> ClickGameFactoryConfig
{
    // game duration
    let game_ticks_per_sec = Ticks(20);
    let game_num_ticks = Ticks(game_ticks_per_sec.0 * 30);

    // versioning
    //todo: use hasher directly
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks  = Ticks(game_ticks_per_sec.0 * 5);
    let game_prep_ticks = Ticks(0);
    let game_over_ticks = Ticks(game_ticks_per_sec.0 * 3);

    // server setup config
    let server_setup_config = GameServerSetupConfig{
            protocol_id,
            expire_secs  : 10u64,
            timeout_secs : 5i32,
            server_ip    : Ipv6Addr::LOCALHOST,
        };

    // game framework config
    let game_fw_config = GameFWConfig::new(game_ticks_per_sec, max_init_ticks, game_over_ticks);

    // game duration config
    let game_duration_config = GameDurationConfig::new(game_prep_ticks, game_num_ticks);

    // click game factory config
    let game_factory_config = ClickGameFactoryConfig{
            server_setup_config,
            game_fw_config,
            game_duration_config,
        };

    game_factory_config
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn run_playtest(num_clients: usize, launch_pack: GameLaunchPack)
{
    // launch in task
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let spawner_clone = spawner.clone();
    let task = spawner.spawn(
        async move
        {
            // launch game
            tracing::trace!("launching game instance for playtest");
            let (game_report_sender, mut game_report_receiver) = new_io_channel::<GameInstanceReport>();
            let game_launcher = GameInstanceLauncherProcess::new(String::from(GAME_INSTANCE_PATH), spawner_clone.clone());
            let mut game_instance = game_launcher.launch(launch_pack, game_report_sender);

            // wait for game start report
            let Some(GameInstanceReport::GameStart(_, mut report)) = game_report_receiver.recv().await
            else { tracing::error!("failed getting game start report for playtest"); return; };

            // prepare to launch the clients
            let Some(meta) = &report.native_meta
            else { tracing::error!("missing native meta for setting up playtest renet client"); return; };

            // launch game clients
            let mut client_instances = Vec::default();
            for _ in 0..num_clients
            {
                let Some(start_info) = report.start_infos.pop()
                else { tracing::error!("missing start info for playtest"); return; };

                let Ok(token) = new_connect_token_native(meta, get_systime(), start_info.client_id)
                else { tracing::error!("failed producing connect token for playtest"); return; };

                let (_client_command_sender, client_command_receiver) = new_io_channel::<ClientInstanceCommand>();
                let (client_report_sender, _client_report_receiver) = new_io_channel::<ClientInstanceReport>();
                tracing::trace!(start_info.client_id, "launching game client for playtest");
                let client_launcher = ClientInstanceLauncherProcess::new(
                        String::from(GAME_CLIENT_PATH),
                        spawner_clone.clone()
                    );
                client_instances.push(client_launcher.launch(
                        token,
                        start_info,
                        client_command_receiver,
                        client_report_sender
                    ));
            }

            // wait for clients to close
            // - we must wait for client closure to avoid zombie process leak
            for mut client_instance in client_instances
            {
                if !client_instance.get().await
                { tracing::warn!("playtest client instance closed with error"); }
            }

            // command game instance to abort
            // - we assume if the client is closed then the game should die, since this is singleplayer
            // - this will do nothing if the game instance already closed
            let _ = game_instance.send_command(GameInstanceCommand::Abort);

            // wait for game instance to close
            if !game_instance.get().await
            { tracing::warn!("playtest instance closed with error"); }

            // get game instance report
            let Some(GameInstanceReport::GameOver(_, _game_over_report)) = game_report_receiver.recv().await
            else { tracing::error!("did not receive game over report for playtest"); return; };
        }
    );

    let _ = enfync::blocking::extract(task);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // logging
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::TRACE.into())
        .from_env().unwrap();
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    // set asset directory location
    #[cfg(not(target_family = "wasm"))]
    {
        if let Err(err) = bevy_girk_utils::try_set_bevy_asset_root(2)
        {
            panic!("Could not set bevy asset root: {}", err.to_string());
        }
    }

    // env
    let args = PlaytestCli::parse();
    tracing::trace!(?args);
    let num_clients = args.clients.unwrap_or(1usize).max(1usize);

    // lobby contents
    let mut players = Vec::default();
    for idx in 0..num_clients
    {
        players.push((bevy_simplenet::EnvType::Native, idx as u128));
    }

    let lobby_contents = ClickLobbyContents{
        id       : 0u64,
        owner_id : 0u128,
        config   : ClickLobbyConfig{
            max_players  : num_clients as u16,
            max_watchers : 0u16,
        },
        players,
        watchers : Vec::default(),
    };

    // launch pack
    let game_configs = make_click_game_configs();
    let Ok(launch_pack) = get_launch_pack(ser_msg(&game_configs), lobby_contents)
    else { tracing::error!("failed getting launch pack for playtest"); return; };

    // run it
    run_playtest(num_clients, launch_pack);
}

//-------------------------------------------------------------------------------------------------------------------
