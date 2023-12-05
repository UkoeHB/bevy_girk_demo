//local shortcuts
use crate::*;
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_kot::prelude::*;
use enfync::{AdoptOrDefault, Handle};

//standard shortcuts
use std::net::Ipv6Addr;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_click_game_configs(game_ticks_per_sec: Ticks, game_num_ticks: Ticks) -> ClickGameFactoryConfig
{
    // versioning
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks  = Ticks(200);
    let game_prep_ticks = Ticks(0);

    // server setup config
    let server_setup_config = GameServerSetupConfig{
            protocol_id,
            expire_seconds  : 10u64,
            timeout_seconds : 5i32,
            server_ip       : Ipv6Addr::LOCALHOST,
        };

    // game framework config
    let game_fw_config = GameFWConfig::new(game_ticks_per_sec, max_init_ticks);

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

pub(crate) struct GameMonitorLocalNative
{
    task: enfync::PendingResult<Option<GameOverReport>>,
}

//-------------------------------------------------------------------------------------------------------------------

impl GameMonitorImpl for GameMonitorLocalNative
{
    fn is_running(&self) -> bool
    {
        !self.task.done()
    }

    fn take_result(&mut self) -> Result<Option<GameOverReport>, ()>
    {
        if self.is_running() { return Err(()); }
        self.task.try_extract()
            .unwrap_or_else(|| Ok(None))
            .or_else(|_| Ok(None))
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Launches a local single-player game.
pub(crate) fn launch_local_player_game_native(lobby_contents: ClickLobbyContents) -> GameMonitorLocalNative
{
    // launch in task
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let task = spawner.spawn(
        async move
        {
            // prep game launch pack
            let game_ticks_per_sec = Ticks(20);
            let game_num_ticks = Ticks(20 * 25);
            let game_configs = make_click_game_configs(game_ticks_per_sec, game_num_ticks);
            let Ok(launch_pack) = get_launch_pack(ser_msg(&game_configs), lobby_contents) else { return None; };

            // launch game
            //todo: inject game app binary path
            let (report_sender, mut report_receiver) = new_io_channel::<GameInstanceReport>();
            let launcher = GameInstanceLauncherProcess::new(String::from("game_instance"));
            let mut game_instance = launcher.launch(launch_pack, report_sender);

            // wait for game start report
            let Some(GameInstanceReport::GameStart(_, report)) = report_receiver.recv().await else { return None; };

            // extract game connect info
            let Some(connect_info) = report.connect_infos.get(0) else { return None; };

            // launch game client
            //todo: inject game client binary path
            let Ok(mut game_client_process) = launch_game_client(String::from("game_client"), connect_info) else { return None; };

            // wait for client to close
            // - it is mandatory to wait for client closure to avoid zombie process leak
            let _ = game_client_process.wait().await;

            // command game instance to abort
            // - we assume if the client is closed then the game should die, since this is singleplayer
            // - this will do nothing if the game instance already closed
            let _ = game_instance.send_command(GameInstanceCommand::AbortGame);

            // wait for game instance to close
            let _ = game_instance.get().await;

            // get game instance report
            let Some(GameInstanceReport::GameOver(_, game_over_report)) = report_receiver.try_recv() else { return None; };

            Some(game_over_report)
        }
    );

    GameMonitorLocalNative{ task }
}

//-------------------------------------------------------------------------------------------------------------------
