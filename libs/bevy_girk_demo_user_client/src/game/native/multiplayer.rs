//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use enfync::{AdoptOrDefault, Handle};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct GameMonitorMultiplayerNative
{
    game_id: u64,
    task: enfync::PendingResult<()>,
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
        //todo
        tracing::error!("kill not yet implemented for GameMonitorMultiplayerNative");
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
pub(crate) fn launch_multiplayer_game_native(game_id: u64, connect_info: GameConnectInfo) -> GameMonitorMultiplayerNative
{
    // launch in task
    let spawner = enfync::builtin::native::TokioHandle::adopt_or_default();
    let task = spawner.spawn(
        async move
        {
            // launch game client
            //todo: inject game client binary path
            tracing::trace!("launching game client for multiplayer game");
            let Ok(mut game_client_process) = launch_game_client(String::from(GAME_CLIENT_PATH), &connect_info)
            else { tracing::error!("failed launching game client for multiplayer game"); return; };

            // wait for client to close
            // - we must wait for client closure to avoid zombie process leak
            let _ = game_client_process.wait().await;

            tracing::info!("multiplayer game ended");
        }
    );

    GameMonitorMultiplayerNative{ game_id, task }
}

//-------------------------------------------------------------------------------------------------------------------
