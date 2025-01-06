use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_kot_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<Receiver<PlayerInput>>() {
        panic!("Receiver<PlayerClientInput> is missing on startup!");
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerInputSet;

//-------------------------------------------------------------------------------------------------------------------

/// Player input plugin.
///
/// Sets up systems for marshalling player inputs to the game instance.
pub(crate) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(PreStartup, prestartup_check)
            .add_systems(Update, handle_player_inputs.in_set(PlayerInputSet))
            .add_systems(OnEnter(ClientInstanceState::Game), clear_player_inputs);
    }
}

//-------------------------------------------------------------------------------------------------------------------
