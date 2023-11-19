//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::*;

//standard shortcuts

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn process_player_client_inputs<F>(world: &mut World, handler: F)
where
    F: Fn(&mut World, &PlayerClientInput)
{
    let Some(player_inputs) = world.remove_resource::<Receiver<PlayerClientInput>>() else { return; };

    while let Some(input) = player_inputs.try_recv()
    {
        handler(world, &input);
    }

    world.insert_resource(player_inputs);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_input_init(_world: &mut World, input: &PlayerClientInput)
{
    let PlayerClientInput::Init(init_input) = input
    else { tracing::warn!(?input, "ignoring invalid input sent during init mode"); return; };

    match init_input
    {
        PlayerClientInputInit::None => ()
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_input_prep(_world: &mut World, input: &PlayerClientInput)
{
    let PlayerClientInput::Prep(prep_input) = input
    else { tracing::warn!(?input, "ignoring invalid input sent during prep mode"); return; };

    match prep_input
    {
        PlayerClientInputPrep::None => ()
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_input_play(world: &mut World, input: &PlayerClientInput)
{
    let PlayerClientInput::Play(play_input) = input
    else { tracing::warn!(?input, "ignoring invalid input sent during play mode"); return; };

    match play_input
    {
        PlayerClientInputPlay::PlayerInput(i) => syscall(world, GameRequest::PlayerInput(*i), send_game_request),
        PlayerClientInputPlay::None           => ()
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn handle_input_gameover(_world: &mut World, input: &PlayerClientInput)
{
    let PlayerClientInput::GameOver(gameover_input) = input
    else { tracing::warn!(?input, "ignoring invalid input sent during gameover mode"); return; };

    match gameover_input
    {
        PlayerClientInputGameOver::None => ()
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientCoreMode::Init.
pub(crate) fn handle_player_client_inputs_init(world: &mut World)
{
    process_player_client_inputs(world, | world, input | handle_input_init(world, input));
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientCoreMode::Prep.
pub(crate) fn handle_player_client_inputs_prep(world: &mut World)
{
    process_player_client_inputs(world, | world, input | handle_input_prep(world, input));
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientCoreMode::Play.
pub(crate) fn handle_player_client_inputs_play(world: &mut World)
{
    process_player_client_inputs(world, | world, input | handle_input_play(world, input));
}

//-------------------------------------------------------------------------------------------------------------------

/// Handle player inputs for ClientCoreMode::GameOver.
pub(crate) fn handle_player_client_inputs_gameover(world: &mut World)
{
    process_player_client_inputs(world, | world, input | handle_input_gameover(world, input));
}

//-------------------------------------------------------------------------------------------------------------------
