use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn setup_client_tag_entities(mut c: Commands)
{
    c.spawn(ReconnectorButton);
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug)]
pub(crate) struct ReconnectorButton;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct HostClientPlugin;

impl Plugin for HostClientPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(HostClientConnectPlugin)
            .add_plugins(HostIncomingPlugin)
            .configure_sets(First, (HandleHostIncomingSet, HostClientConnectSet).chain())
            .add_systems(PreStartup, setup_client_tag_entities);
    }
}

//-------------------------------------------------------------------------------------------------------------------
