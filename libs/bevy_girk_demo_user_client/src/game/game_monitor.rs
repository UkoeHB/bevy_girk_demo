//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_game_fw::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn check_game_monitor(mut rcommands: ReactCommands, mut monitor: ReactResMut<GameMonitor>)
{
    // do nothing if the game is running
    if monitor.is_running() { return; }

    // try to extract the game over report
    let Ok(result) = monitor.get_mut_noreact().take_result() else { return; };

    // send game over report to the app
    if let Some(report) = result
    {
        rcommands.send(report);
    }

    // clear the finished game
    monitor.get_mut(&mut rcommands).clear();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) trait GameMonitorImpl
{
    /// Check if the game is running.
    fn is_running(&self) -> bool;
    /// Try to take the result.
    /// - Returns `Err` if the game is still running.
    /// - Returns `Some(None)` if the game is over but there is no report available (either it doesn't exist or it was
    ///   already extracted).
    ///
    /// Note that this method is mainly designed for single-player games.
    fn take_result(&mut self) -> Result<Option<GameOverReport>, ()>;
}

//-------------------------------------------------------------------------------------------------------------------

/// Monitors an ongoing game if one exists.
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
        let Some(inner) = &mut self.inner else { return Ok(None); };
        inner.take_result()
    }
}

impl Default for GameMonitor { fn default() -> Self { Self{ inner: None } } }

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn GameMonitorPlugin(app: &mut App)
{
    app.insert_react_resource(GameMonitor::default())
        .add_systems(First, check_game_monitor);
}

//-------------------------------------------------------------------------------------------------------------------
