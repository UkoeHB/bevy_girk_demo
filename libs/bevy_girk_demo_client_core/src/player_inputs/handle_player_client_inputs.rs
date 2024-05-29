use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_demo_game_core::*;
use bevy_kot_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn process_player_inputs(world: &mut World, handler: impl Fn(&mut World, PlayerClientInput))
{
    let Some(player_inputs) = world.remove_resource::<Receiver<PlayerClientInput>>() else {
        return;
    };

    while let Some(input) = player_inputs.try_recv() {
        if world.resource::<ClientContext>().client_type() != ClientType::Player {
            tracing::warn!(?input, "ignoring input sent by a non-player client");
            continue;
        }

        handler(world, input);
    }

    world.insert_resource(player_inputs);
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_input_init(_world: &mut World, input: PlayerClientInput)
{
    let PlayerClientInput::Init = input else {
        tracing::warn!(?input, "ignoring invalid input sent during init mode");
        return;
    };
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_input_prep(_world: &mut World, input: PlayerClientInput)
{
    let PlayerClientInput::Prep = input else {
        tracing::warn!(?input, "ignoring invalid input sent during prep mode");
        return;
    };
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_input_play(world: &mut World, input: PlayerClientInput)
{
    let PlayerClientInput::Play(player_input) = input else {
        tracing::warn!(?input, "ignoring invalid input sent during play mode");
        return;
    };

    syscall(world, ClientRequest::PlayerInput(player_input), send_client_request);
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_input_gameover(_world: &mut World, input: PlayerClientInput)
{
    let PlayerClientInput::GameOver = input else {
        tracing::warn!(?input, "ignoring invalid input sent during gameover mode");
        return;
    };
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientMode::Init.
pub(crate) fn handle_player_inputs_init(world: &mut World)
{
    process_player_inputs(world, |world, input| handle_input_init(world, input));
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientMode::Prep.
pub(crate) fn handle_player_inputs_prep(world: &mut World)
{
    process_player_inputs(world, |world, input| handle_input_prep(world, input));
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientMode::Play.
pub(crate) fn handle_player_inputs_play(world: &mut World)
{
    process_player_inputs(world, |world, input| handle_input_play(world, input));
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientMode::GameOver.
pub(crate) fn handle_player_inputs_gameover(world: &mut World)
{
    process_player_inputs(world, |world, input| handle_input_gameover(world, input));
}

//-------------------------------------------------------------------------------------------------------------------
