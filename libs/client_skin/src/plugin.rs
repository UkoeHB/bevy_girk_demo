use bevy::prelude::*;
use bevy_cobweb::prelude::*;

use super::*;

//-------------------------------------------------------------------------------------------------------------------

/// Plugin for setting up a click demo game client skin.
///
/// Prerequisites:
/// - Use `make_game_client_core()` to set up a client app.
pub struct ClientSkinPlugin;

impl Plugin for ClientSkinPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(ReactPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(FpsTrackerPlugin)
            .add_plugins(LoadingSimPlugin)
            .add_plugins(StateChangesPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
