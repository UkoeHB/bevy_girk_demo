//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_kot_ui::InteractionSourceSet;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn HostClientPlugin(app: &mut App)
{
    app
        .insert_react_resource(ConnectionStatus::Connecting)
        .add_systems(PreStartup, setup_client_tag_entities)
        .add_systems(First,
            (
                handle_host_incoming,
                reaction_tree,
            ).chain()
                .before(InteractionSourceSet)
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
