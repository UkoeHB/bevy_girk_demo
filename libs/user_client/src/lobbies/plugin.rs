use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct LobbiesPlugin;

impl Plugin for LobbiesPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(AckRequestPlugin)
            .add_plugins(LobbyDisplayPlugin)
            .add_plugins(LobbyPagePlugin)
            .add_systems(PreStartup, setup_lobby_tag_entities);
    }
}

//-------------------------------------------------------------------------------------------------------------------
