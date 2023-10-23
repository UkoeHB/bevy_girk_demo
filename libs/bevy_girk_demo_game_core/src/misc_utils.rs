//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_girk_utils::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Produce a new PRNG for a specific player.
pub fn make_player_rand(domain_sep: &str, seed: u64, player_id: PlayerId) -> Rand64
{
    let shifted_seed = (seed as u128).checked_shl(64).unwrap();
    let player_seed  = shifted_seed + ((player_id.id as u64) as u128);

    Rand64::new(domain_sep, player_seed)
}

//-------------------------------------------------------------------------------------------------------------------
