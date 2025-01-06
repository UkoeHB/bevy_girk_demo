use bevy::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

fn advance_game_tick(mut game_tick: ResMut<GameTick>)
{
    *game_tick.0 += 1;
}

//-------------------------------------------------------------------------------------------------------------------

fn advance_prep_tick(mut prep_tick: ResMut<PrepTick>)
{
    *prep_tick.0 += 1;
}

//-------------------------------------------------------------------------------------------------------------------

fn advance_play_tick(mut play_tick: ResMut<PlayTick>)
{
    *play_tick.0 += 1;
}

//-------------------------------------------------------------------------------------------------------------------

fn advance_game_over_tick(mut game_over_tick: ResMut<GameOverTick>)
{
    *game_over_tick.0 += 1;
}

//-------------------------------------------------------------------------------------------------------------------

/// The current global game tick since the game began (after the game was initialized).
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct GameTick(pub Tick);

/// The current tick while in [GameState::Prep].
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct PrepTick(pub Tick);

/// The current tick while in [GameState::Play].
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct PlayTick(pub Tick);

/// The current tick while in [GameState::GameOver].
#[derive(Resource, Default, Debug, Copy, Clone, Deref)]
pub struct GameOverTick(pub Tick);

//-------------------------------------------------------------------------------------------------------------------

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TickUpdateSet;

//-------------------------------------------------------------------------------------------------------------------

/// Game tick plugin.
pub(crate) struct GameTickPlugin;

impl Plugin for GameTickPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .init_resource::<GameTick>()
            .init_resource::<PrepTick>()
            .init_resource::<PlayTick>()
            .init_resource::<GameOverTick>()
            .add_systems(
                Update,
                (
                    advance_game_tick.in_set(GameSet::PostInit),
                    advance_prep_tick.in_set(GameSet::Prep),
                    advance_play_tick.in_set(GameSet::Play),
                    advance_game_over_tick.in_set(GameSet::GameOver),
                )
                    .chain()
                    .in_set(TickUpdateSet),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
