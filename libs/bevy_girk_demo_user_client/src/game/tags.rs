//local shortcuts

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug)]
pub(crate) struct ConnectTokenRequest;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn setup_game_tag_entities(mut commands: Commands)
{
    commands.spawn(ConnectTokenRequest);
}

//-------------------------------------------------------------------------------------------------------------------
