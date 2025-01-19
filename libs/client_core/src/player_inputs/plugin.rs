use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_utils::Receiver;
use game_core::PlayerInput;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<Receiver<PlayerInput>>() {
        tracing::error!("Receiver<PlayerClientInput> is missing on game startup!");
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
        app.add_systems(OnEnter(ClientAppState::Game), prestartup_check)
            .add_systems(Update, handle_player_inputs.in_set(PlayerInputSet))
            .add_systems(OnEnter(ClientAppState::Game), clear_player_inputs);
    }
}

//-------------------------------------------------------------------------------------------------------------------
