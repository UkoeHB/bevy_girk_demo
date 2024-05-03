use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn LobbiesPlugin(app: &mut App)
{
    app.add_plugins(AckRequestPlugin)
        .add_plugins(LobbyDisplayPlugin)
        .add_plugins(LobbyPagePlugin)
        .add_systems(PreStartup, setup_lobby_tag_entities);
}

//-------------------------------------------------------------------------------------------------------------------
