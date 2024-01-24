//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_girk_utils::*;
use serde::{Serialize, Deserialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// The game's deterministic random number generator
#[derive(Resource)]
pub struct GameRand(Rand64);

impl GameRand
{
    pub fn new(seed: u128) -> GameRand
    {
        GameRand(Rand64::new("bevy_girk_demo", seed))
    }

    pub fn next(&mut self) -> u64 { self.0.next() }
}

//-------------------------------------------------------------------------------------------------------------------

/// The current global game tick since the game began (after the game was initialized).
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct GameTick(pub Tick);

/// The curren tick while in [GameMode::Prep].
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct PrepTick(pub Tick);

/// The curren tick while in [GameMode::Play].
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct PlayTick(pub Tick);

/// The curren tick while in [GameMode::GameOver].
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct GameOverTick(pub Tick);

//-------------------------------------------------------------------------------------------------------------------

/// Game mode
#[derive(States, Debug, Default, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum GameMode
{
    #[default]
    Init,
    Prep,
    Play,
    GameOver
}

//-------------------------------------------------------------------------------------------------------------------
