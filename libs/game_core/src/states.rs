use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------------------------

/// Check the game duration conditions and update the game state.
fn update_game_state(
    game_ctx: Res<ClickGameContext>,
    game_tick: Res<GameTick>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
)
{
    // get expected state based on elapsed ticks
    let duration_config = game_ctx.duration_config();
    let new_game_state = duration_config.expected_state(**game_tick);

    // update the game state
    if new_game_state == **current_game_state {
        return;
    }
    next_game_state.set(new_game_state);
    tracing::info!(?new_game_state, "new game state");
}

//-------------------------------------------------------------------------------------------------------------------

fn set_game_end_flag(
    game_tick: Res<GameTick>,
    players: Query<(&PlayerId, &PlayerScore)>,
    mut game_end_flag: ResMut<GameEndFlag>,
)
{
    // collect player reports
    let player_reports = players
        .iter()
        .map(|(&player_id, &score)| ClickPlayerReport { client_id: player_id.id, score })
        .collect();

    // build game over report
    let game_over_report = ClickGameOverReport { final_game_tick: **game_tick, player_reports };

    // serialize it
    let game_over_report_final = GameOverReport::new(ser_msg(&game_over_report));

    // set the game end flag
    game_end_flag.set(game_over_report_final);
    tracing::info!("game end flag set");
}

//-------------------------------------------------------------------------------------------------------------------

/// Notify all clients of the current game state.
pub(crate) fn notify_game_state_all(
    game_state: Res<State<GameState>>,
    mut sender: GameMessageSender,
    attributes: ClientAttributes,
)
{
    sender.send(&attributes, GameMsg::CurrentGameState(**game_state), vis!(Global));
}

//-------------------------------------------------------------------------------------------------------------------

/// Helper function-system for accessing the game state.
pub(crate) fn get_game_state(game_state: Res<State<GameState>>) -> GameState
{
    **game_state
}

//-------------------------------------------------------------------------------------------------------------------

/// Notify a single client of the current game state.
pub(crate) fn notify_game_state_single(
    In(client_id): In<ClientId>,
    game_state: Res<State<GameState>>,
    mut sender: GameMessageSender,
    attributes: ClientAttributes,
)
{
    sender.send(
        &attributes,
        GameMsg::CurrentGameState(**game_state),
        vis!(Client(client_id)),
    );
}

//-------------------------------------------------------------------------------------------------------------------

/// Game state
#[derive(States, Debug, Default, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum GameState
{
    #[default]
    Init,
    Prep,
    Play,
    GameOver,
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameStateUpdateSet;

//-------------------------------------------------------------------------------------------------------------------

/// Game tick plugin.
pub(crate) struct GameStatePlugin;

impl Plugin for GameStatePlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_state::<GameState>()
            .add_systems(
                Update,
                (
                    // determine which game mode the previous tick was in and set it
                    update_game_state,
                    apply_state_transitions,
                )
                    .chain()
                    .in_set(GameSet::PostInit)
                    .in_set(GameStateUpdateSet),
            )
            .add_systems(OnEnter(GameState::Init), notify_game_mode_all)
            .add_systems(OnEnter(GameState::Prep), notify_game_mode_all)
            .add_systems(OnEnter(GameState::Play), notify_game_mode_all)
            .add_systems(OnEnter(GameState::GameOver), (notify_game_state_all, set_game_end_flag));
    }
}

//-------------------------------------------------------------------------------------------------------------------
