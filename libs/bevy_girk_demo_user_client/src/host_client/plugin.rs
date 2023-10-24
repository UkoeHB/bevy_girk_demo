//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn HostClientPlugin(app: &mut App)
{
    app
        .insert_resource(ConnectionStatus::Connecting)
        .add_systems(PreUpdate,
            (
                handle_connection_changes,
                handle_host_incoming,
            ).chain()
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
