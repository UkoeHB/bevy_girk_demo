//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn HostClientPlugin(app: &mut App)
{
    app
        .insert_react_resource(ConnectionStatus::Connecting)
        .add_react_event::<bevy_girk_game_fw::GameOverReport>()
        .add_systems(First,
            (
                handle_connection_changes,
                handle_host_incoming,
                react_to_all_removals_and_despawns,
            ).chain()
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
