//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_game_fw::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Clear the game monitor when the monitor has a result.
fn cleanup_game_monitor(mut rcommands: ReactCommands, mut monitor: ReactResMut<GameMonitor>)
{
    // check if the monitor may have a result
    if monitor.is_running() || !monitor.has_game() { return; }

    // try to extract the game over report
    let Ok(result) = monitor.get_mut_noreact().take_result() else { return; };

    // send game over report to the app
    if let Some(report) = result
    {
        tracing::info!("received game over report from game monitor");
        rcommands.send(report);
    }

    // clear the finished game
    monitor.get_mut(&mut rcommands).clear();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) trait GameMonitorImpl
{
    /// Get the current game's id.
    fn game_id(&self) -> u64;
    /// Check if the game client is running.
    fn is_running(&self) -> bool;
    /// Kill the current game client.
    fn kill(&mut self);
    /// Try to take the result.
    /// - Returns `Err` if the game is still running.
    /// - Returns `Some(None)` if the game is over but there is no report available (either it doesn't exist or it was
    ///   already extracted).
    ///
    /// Note that this method is mainly designed for single-player games.
    fn take_result(&mut self) -> Result<Option<GameOverReport>, ()>;
}

//-------------------------------------------------------------------------------------------------------------------

/// Monitors an ongoing game client if one exists.
#[derive(ReactResource)]
pub(crate) struct GameMonitor
{
    inner: Option<Box<dyn GameMonitorImpl + Send + Sync + 'static>>,
}

impl GameMonitor
{
    pub(crate) fn set(&mut self, monitor: impl GameMonitorImpl + Send + Sync + 'static)
    {
        self.inner = Some(Box::new(monitor));
    }

    pub(crate) fn game_id(&self) -> Option<u64>
    {
        let Some(inner) = &self.inner else { return None; };
        Some(inner.game_id())
    }

    pub(crate) fn kill(&mut self, game_id: u64)
    {
        let Some(inner) = &mut self.inner else { return; };
        if inner.game_id() != game_id { return; };
        inner.kill();
    }

    pub(crate) fn clear(&mut self)
    {
        self.inner = None;
    }

    pub(crate) fn has_game(&self) -> bool
    {
        self.inner.is_some()
    }

    pub(crate) fn is_running(&self) -> bool
    {
        let Some(inner) = &self.inner else { return false; };
        inner.is_running()
    }

    pub(crate) fn take_result(&mut self) -> Result<Option<GameOverReport>, ()>
    {
        let Some(inner) = &mut self.inner else { return Err(()); };
        inner.take_result()
    }
}

impl Default for GameMonitor { fn default() -> Self { Self{ inner: None } } }

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn GameMonitorPlugin(app: &mut App)
{
    app.insert_react_resource(GameMonitor::default())
        .add_systems(First, cleanup_game_monitor);
}

//-------------------------------------------------------------------------------------------------------------------
