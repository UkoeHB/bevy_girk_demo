//local shortcuts
use crate::*;
use bevy_girk_demo_wiring_client_instance::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use enfync::{AdoptOrDefault, Handle};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct GameMonitorMultiplayerNative
{
    game_id             : u64,
    task                : enfync::PendingResult<()>,
    abort_signal_sender : Option<tokio::sync::oneshot::Sender<()>>,
}

//-------------------------------------------------------------------------------------------------------------------

impl GameMonitorImpl for GameMonitorMultiplayerNative
{
    fn game_id(&self) -> u64
    {
        self.game_id
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
        Ok(None)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Launches a multi-player game.
//todo: inject all configs
pub(crate) fn launch_multiplayer_game_native(
    game_id    : u64,
    token      : ServerConnectToken,
    start_info : GameStartInfo
) -> GameMonitorMultiplayerNative
{
    // launch in task
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let (abort_signal_sender, abort_signal_receiver) = tokio::sync::oneshot::channel::<()>();
    let task = spawner.spawn(
        async move
        {
            // launch game client
            //todo: inject game client binary path
            tracing::trace!("launching game client for multiplayer game");
            let Ok(mut game_client_process) = launch_game_client(String::from(GAME_CLIENT_PATH), &token, &start_info)
            else { tracing::error!("failed launching game client for multiplayer game"); return; };

            // wait for client to close
            // - we must wait for client closure to avoid zombie process leak
            tokio::select!
            {
                _ = game_client_process.wait() => tracing::debug!("multi-player game client closed"),
                _ = abort_signal_receiver =>
                {
                    let _ = game_client_process.kill().await;
                    tracing::warn!("multi-player game client killed");
                }
            }

            tracing::info!("multiplayer game ended");
        }
    );

    GameMonitorMultiplayerNative{ game_id, task, abort_signal_sender: Some(abort_signal_sender) }
}

//-------------------------------------------------------------------------------------------------------------------
