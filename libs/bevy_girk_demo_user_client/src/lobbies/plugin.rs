//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

//standard shortcuts

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn LobbiesPlugin(app: &mut App)
{
    app
        .add_systems(PreStartup, setup_lobby_button_entities)
        .add_plugins(AckRequestPlugin)
        .add_plugins(LobbyDisplayPlugin)
        .add_plugins(LobbyPagePlugin)
        .add_plugins(PendingLobbyResetPlugin)
        ;
}

//-------------------------------------------------------------------------------------------------------------------
