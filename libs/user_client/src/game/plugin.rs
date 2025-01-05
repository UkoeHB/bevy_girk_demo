use bevy::prelude::*;
use bevy_girk_user_client_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(ClientMonitorPlugin)
            .add_plugins(ClientStarterPlugin)
            .add_systems(PreStartup, setup_game_tag_entities)
            .add_systems(First, handle_client_instance_reports);
    }
}

//-------------------------------------------------------------------------------------------------------------------
