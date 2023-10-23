//local shortcuts
use bevy_girk_demo_game_core::*;

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Inputs that may come from a player client during Init mode.
#[derive(Debug)]
pub enum PlayerClientInputInit
{
    None,
}

//-------------------------------------------------------------------------------------------------------------------

/// Inputs that may come from a player client during Prep mode.
#[derive(Debug)]
pub enum PlayerClientInputPrep
{
    None,
}

//-------------------------------------------------------------------------------------------------------------------

/// Inputs that may come from a player client during Play mode.
#[derive(Debug)]
pub enum PlayerClientInputPlay
{
    PlayerInput(PlayerInput),
    None,
}

//-------------------------------------------------------------------------------------------------------------------

/// Inputs that may come from a player client during GameOver mode.
#[derive(Debug)]
pub enum PlayerClientInputGameOver
{
    None,
}

//-------------------------------------------------------------------------------------------------------------------

/// Inputs that may come from a player client.
#[derive(Debug)]
pub enum PlayerClientInput
{
    Init(PlayerClientInputInit),
    Prep(PlayerClientInputPrep),
    Play(PlayerClientInputPlay),
    GameOver(PlayerClientInputGameOver),
}

//-------------------------------------------------------------------------------------------------------------------
