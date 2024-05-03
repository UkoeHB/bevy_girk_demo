use bevy_girk_demo_game_core::*;

//-------------------------------------------------------------------------------------------------------------------

/// Inputs that may come from a player client.
#[derive(Debug)]
pub enum PlayerClientInput
{
    Init,
    Prep,
    Play(PlayerInput),
    GameOver,
}

//-------------------------------------------------------------------------------------------------------------------
