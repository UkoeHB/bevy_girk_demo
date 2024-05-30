use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_kot_ui::InteractionSourceSet;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct HostClientPlugin;

impl Plugin for HostClientPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_react_resource(ConnectionStatus::Connecting)
            .add_systems(PreStartup, setup_client_tag_entities)
            .add_systems(
                First,
                (handle_host_incoming, reaction_tree)
                    .chain()
                    .before(InteractionSourceSet),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
