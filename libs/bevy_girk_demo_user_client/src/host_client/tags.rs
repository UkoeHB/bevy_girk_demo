//local shortcuts

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug)]
pub(crate) struct ReconnectorButton;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn setup_client_tag_entities(mut commands: Commands)
{
    commands.spawn(ReconnectorButton);
}

//-------------------------------------------------------------------------------------------------------------------
