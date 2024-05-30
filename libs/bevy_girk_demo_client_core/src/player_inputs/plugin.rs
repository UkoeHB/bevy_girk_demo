use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_kot_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<Receiver<PlayerClientInput>>() {
        panic!("Receiver<PlayerClientInput> is missing on startup!");
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Player input plugin.
///
/// Sets up systems for marshalling player inputs to the game instance.
pub struct PlayerClientInputPlugin;

impl Plugin for PlayerClientInputPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(PreStartup, (prestartup_check,))
            .add_systems(
                Update,
                (
                    handle_player_inputs_init.in_set(ClientSet::InitStartup),
                    handle_player_inputs_prep.in_set(ClientSet::Prep),
                    handle_player_inputs_play.in_set(ClientSet::Play),
                    handle_player_inputs_gameover.in_set(ClientSet::GameOver),
                )
                    .chain()
                    .in_set(ClientFwSet::Admin),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
