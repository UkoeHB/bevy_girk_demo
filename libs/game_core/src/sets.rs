use bevy::prelude::*;
use bevy_girk_game_fw::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// System sets that contain tick game logic. These don't run during initialization.
///
/// These sets are modal. Use [`GameFwSet`] for ordinal control.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet
{
    /// Runs in [`Update`] when not in [`GameFwStateInit`].
    PostInit,
    Prep,
    Play,
    GameOver,
    End,
}

//-------------------------------------------------------------------------------------------------------------------

/// Game core logic sets.
///
/// These sets are ordinal in schedule `Update`.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameLogicSet
{
    Admin,
    Update,
}

//-------------------------------------------------------------------------------------------------------------------

/// Configures root-level system sets.
pub struct GameSetsPlugin;

impl Plugin for GameSetsPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .configure_sets(
                Update,
                GameSet::PostInit
                    .run_if(not(in_state(GameFwState::Init)))
            )
            .configure_sets(
                Update,
                GameSet::Prep
                    .run_if(in_state(GameFwState::Game))
                    .run_if(in_state(GameState::Prep)),
            )
            .configure_sets(
                Update,
                GameSet::Play
                    .run_if(in_state(GameFwState::Game))
                    .run_if(in_state(GameState::Play)),
            )
            // - This will only run in the span between entering 'game over' and the GameFwState moving to 'End', which
            //   is controlled by `GameFwConfig::max_end_ticks()`.
            //todo: allow GameOver to last indefinitely?
            .configure_sets(
                Update,
                GameSet::GameOver
                    .run_if(in_state(GameFwState::Game))
                    .run_if(in_state(GameState::GameOver)),
            )
            .configure_sets(
                Update,
                (
                    GameLogicSet::Admin,
                    GameLogicSet::Update
                )
                    .chain()
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
