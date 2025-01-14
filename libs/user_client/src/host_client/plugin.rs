use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct HostClientPlugin;

impl Plugin for HostClientPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(HostClientConnectPlugin)
            .add_plugins(HostIncomingPlugin)
            .configure_sets(
                First,
                (HandleHostIncomingSet, HostClientConnectSet)
                    .chain()
                    .run_if(not(in_state(ClientAppState::Loading))),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
