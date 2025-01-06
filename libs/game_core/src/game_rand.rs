use bevy_girk_game_fw::*;
use bevy_girk_utils::*;

use crate::*;

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

    pub fn next(&mut self) -> u64
    {
        self.0.next()
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Produce a new PRNG for a specific player.
pub fn make_player_rand(domain_sep: &str, seed: u64, player_id: PlayerId) -> Rand64
{
    let shifted_seed = (seed as u128).checked_shl(64).unwrap();
    let player_seed = shifted_seed + (player_id.id.get() as u128);

    Rand64::new(domain_sep, player_seed)
}

//-------------------------------------------------------------------------------------------------------------------
