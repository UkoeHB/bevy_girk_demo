//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn HostClientPlugin(app: &mut App)
{
    app
        .insert_resource(ReactRes::new(ConnectionStatus::Connecting))
        .add_systems(First,
            (
                handle_connection_changes,
                handle_host_incoming,
                bevy_kot::ecs::react_to_all_removals_and_despawns,
            ).chain()
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
