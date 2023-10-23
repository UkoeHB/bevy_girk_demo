//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Initialize the client state.
pub(crate) fn setup_player_state(world: &mut World)
{
    let player_initializer = world.remove_resource::<ClickPlayerInitializer>().expect("initializer missing");
    world.insert_resource(player_initializer.player_context);
}

//-------------------------------------------------------------------------------------------------------------------
