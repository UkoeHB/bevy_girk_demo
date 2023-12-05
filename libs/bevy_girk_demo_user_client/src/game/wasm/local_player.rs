//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/*

- game instance app
- game client app with shared-process connect info
- run apps concurrently
    - game client takes over renderer

*/

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct GameMonitorLocalWasm
{}

//-------------------------------------------------------------------------------------------------------------------

impl GameMonitorImpl for GameMonitorLocalWasm
{
    pub(crate) fn is_running(&self) -> bool
    {
        false
    }

    pub(crate) fn take_result(&mut self) -> Result<Option<GameOverReport>, ()>
    {
        Ok(None)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Launches a local single-player game.
pub(crate) fn launch_local_player_game_wasm(_lobby_contents: ClickLobbyContents) -> GameMonitorLocalWasm
{
    GameMonitorLocalWasm
}

//-------------------------------------------------------------------------------------------------------------------
