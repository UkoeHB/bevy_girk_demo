//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_fn_plugin::bevy_plugin;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn GamePlugin(app: &mut App)
{
    app.add_plugins(GameMonitorPlugin)
        .add_plugins(GameReconnectorPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
