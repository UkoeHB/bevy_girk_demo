//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_kot::prelude::*;
use enfync::{AdoptOrDefault, Handle};

//standard shortcuts
use std::time::{Duration, SystemTime, UNIX_EPOCH};

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn get_systime() -> Duration
{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default()
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct GameMonitorLocalNative
{
    task                : enfync::PendingResult<Option<GameOverReport>>,
    abort_signal_sender : Option<tokio::sync::oneshot::Sender<()>>,
}

//-------------------------------------------------------------------------------------------------------------------

impl GameMonitorImpl for GameMonitorLocalNative
{
    fn game_id(&self) -> u64
    {
        u64::MAX
    }

    fn is_running(&self) -> bool
    {
        !self.task.done()
    }

    fn kill(&mut self)
    {
        let Some(sender) = self.abort_signal_sender.take() else { return; };
        let _ = sender.send(());
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
//todo: inject all configs
pub(crate) fn launch_local_player_game_native(lobby_contents: ClickLobbyContents) -> GameMonitorLocalNative
{
    // launch in task
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let (abort_signal_sender, abort_signal_receiver) = tokio::sync::oneshot::channel::<()>();
    let task = spawner.spawn(
        async move
        {
            // prep game launch pack
            let game_configs = make_click_game_configs();
            let Ok(launch_pack) = get_launch_pack(ser_msg(&game_configs), lobby_contents)
            else { tracing::error!("failed getting launch pack for local player game"); return None; };

            // launch game
            //todo: inject game app binary path
            tracing::trace!("launching game instance for local player game");
            let (report_sender, mut report_receiver) = new_io_channel::<GameInstanceReport>();
            let launcher = GameInstanceLauncherProcess::new(String::from(GAME_INSTANCE_PATH));
            let mut game_instance = launcher.launch(launch_pack, report_sender);

            // wait for game start report
            let Some(GameInstanceReport::GameStart(_, report)) = report_receiver.recv().await
            else { tracing::error!("failed getting game start report for local player game"); return None; };

            // prepare to launch the client
            let Some(meta) = &report.native_meta
            else { tracing::error!("missing native meta for setting up local player renet client"); return None; };

            let Some(start_info) = report.start_infos.get(0)
            else { tracing::error!("missing start info for local player game"); return None; };

            let Ok(token) = new_connect_token_native(meta, get_systime(), start_info.client_id)
            else { tracing::error!("failed producing connect token for local player game"); return None; };

            // launch game client
            //todo: inject game client binary path
            tracing::trace!("launching game client for local player game");
            let Ok(mut game_client_process) = launch_game_client(String::from(GAME_CLIENT_PATH), &token, start_info)
            else { tracing::error!("failed launching game client for local player game"); return None; };

            // wait for client to close
            // - we must wait for client closure to avoid zombie process leak
            tokio::select!
            {
                _ = game_client_process.wait() => tracing::debug!("local player game client closed"),
                _ = abort_signal_receiver =>
                {
                    let _ = game_client_process.kill().await;
                    tracing::warn!("local player game client killed");
                }
            }

            // command game instance to abort
            // - we assume if the client is closed then the game should die, since this is singleplayer
            // - this will do nothing if the game instance already closed
            let _ = game_instance.send_command(GameInstanceCommand::Abort);

            // wait for game instance to close
            if !game_instance.get().await
            { tracing::warn!("local player game instance closed with error"); }

            // get game instance report
            let Some(GameInstanceReport::GameOver(_, game_over_report)) = report_receiver.recv().await
            else { tracing::error!("did not receive game over report for local player game"); return None; };

            tracing::info!("local player game ended");
            Some(game_over_report)
        }
    );

    GameMonitorLocalNative{ task, abort_signal_sender: Some(abort_signal_sender) }
}

//-------------------------------------------------------------------------------------------------------------------
