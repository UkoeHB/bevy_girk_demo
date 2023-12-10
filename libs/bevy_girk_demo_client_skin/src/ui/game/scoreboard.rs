//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::collections::{BTreeSet, HashMap};
use std::fmt::Write;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

const MAX_PLAYERS: usize = 8;
const ENTRY_HEIGHT: f32 = 100. / MAX_PLAYERS as f32;

macro_rules! PLACEMENT_FMT { () => ("{}.") }
macro_rules! SCORE_FMT     { () => ("{}: {}") }

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

struct TrackerEntry
{
    score_widget : Widget,
    score        : PlayerScore,
}

#[derive(ReactResource, Default)]
struct GameScoreboardTracker
{
    placements: Vec<Widget>,
    entries: HashMap<Entity, TrackerEntry>,
    ordered: BTreeSet<(PlayerScore, Entity)>,
}

impl GameScoreboardTracker
{
    fn add(
        &mut self,
        player_entity    : Entity,
        placement_widget : Widget,
        score_widget     : Widget,
        score            : PlayerScore
    ){
        self.placements.push(placement_widget);
        let _ = self.entries.insert(player_entity, TrackerEntry{ score_widget, score });
        let _ = self.ordered.insert((score, player_entity));
    }

    fn update_score(&mut self, player_entity: Entity, new_score: PlayerScore)
    {
        let Some(entry) = self.entries.get_mut(&player_entity) else { return; };
        let prev_entry = entry.score;
        entry.score = new_score;

        self.ordered.remove(&(prev_entry, player_entity));
        self.ordered.insert((new_score, player_entity));
    }

    fn iter_score_entries(&self) -> impl Iterator<Item = Option<&Widget>> + '_
    {
        self.ordered
            .iter()
            .map(|(_score, entity)|
                self.entries
                    .get(entity)
                    .map(|entry|
                        &entry.score_widget
                    )
            )
    }

    fn pop_entry(&mut self, player_entity: Entity) -> Option<Widget>
    {
        let Some(entry) = self.entries.remove(&player_entity) else { return None; };
        let _ = self.ordered.remove(&(entry.score, player_entity));

        self.placements.pop()
    }

    fn len(&self) -> usize
    {
        self.entries.len()
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_scoreboard(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // scoreboard section
    let scoreboard = relative_widget(ui.tree(), area.end(""), (10., 90.), (5., 95.));

    // dynamically add player when PlayerScore is inserted
    ui.rcommands.on(insertion::<PlayerScore>(),
        move
        |
            In(player_entity) : In<Entity>,
            mut ui            : UiBuilder<MainUi>,
            mut tracker       : ReactResMut<GameScoreboardTracker>,
            ctx               : Res<ClientContext>,
            players           : Query<(&PlayerId, &React<PlayerName>, &React<PlayerScore>)>
        |
        {
            // add style
            ui.style_stack.push();
            ui.add_style(ui.style::<GameScoreboardStyle>().text.clone());

            // access player
            let Ok((id, name, score)) = players.get(player_entity)
            else { tracing::error!(?player_entity, "player entity missing on score insertion"); return; };

            // make scoreboard placement entry (i.e. 1., 2., 3., etc.)
            let num_players = tracker.len() + 1;
            let placement_height = ((num_players - 1) as f32 * ENTRY_HEIGHT, num_players as f32 * ENTRY_HEIGHT);
            let placement_entry = relative_widget(ui.tree(), scoreboard.end(""), (10., 20.), placement_height);
            let placement_text = relative_widget(ui.tree(), placement_entry.end(""), (10., 100.), (10., 90.));

            spawn_basic_text(
                    &mut ui,
                    placement_text,
                    TextParams::centerright(),
                    format!(PLACEMENT_FMT!(), num_players).as_str()
                );

            // make scoreboard entry (i.e. playername: score)
            let score_entry = relative_widget(ui.tree(), scoreboard.end(""), (25., 75.), (0., ENTRY_HEIGHT));
            let score_text = relative_widget(ui.tree(), score_entry.end(""), (10., 90.), (10., 90.));

            let score_text_entity = spawn_basic_text(
                    &mut ui,
                    score_text,
                    TextParams::centerleft()
                        .with_height(Some(70.)),
                    format!(SCORE_FMT!(), name.name, score.score()).as_str()
                );

            // outline score if player id == this user id
            if id.id == ctx.id()
            {
                spawn_plain_outline(&mut ui, score_entry.clone());
            }

            // update score display value when PlayerScore is mutated
            let revoke_token = ui.rcommands.on(entity_mutation::<PlayerScore>(player_entity),
                move
                |
                    mut ui      : UiUtils<MainUi>,
                    players     : Query<(&React<PlayerName>, &React<PlayerScore>), With<PlayerId>>,
                    mut tracker : ReactResMut<GameScoreboardTracker>
                |
                {
                    // access player
                    let Ok((name, score)) = players.get(player_entity)
                    else { tracing::error!(?player_entity, "player entity missing on score mutation"); return; };

                    // update score
                    ui.text.write(score_text_entity, 0, |text| write!(text, SCORE_FMT!(), name.name, score.score())).unwrap();

                    // update tracker
                    tracker.get_mut(&mut ui.builder.rcommands).update_score(player_entity, **score);
                }
            );

            // cleanup
            let score_entry_clone = score_entry.clone();
            let cleanup = move |mut ui: UiUtils<MainUi>, mut tracker: ReactResMut<GameScoreboardTracker>|
            {
                // remove entry from tracker
                // - the placement entry returned is the 'last' position in the scoreboard
                let Some(placement_entry) = tracker.get_mut(&mut ui.builder.rcommands).pop_entry(player_entity)
                else { tracing::warn!(?player_entity, "dead player is missing from scoreboard tracker"); return; };

                // clean up ui
                ui.remove_widget(&placement_entry);
                ui.remove_widget(&score_entry_clone.clone());
                ui.builder.rcommands.revoke(revoke_token.clone());
            };
            //todo: this callback leaks if PlayerScore is added/removed many times (need to add once() adaptor to bevy_kot) 
            ui.rcommands.on(entity_removal::<PlayerScore>(player_entity), cleanup.clone());
            ui.rcommands.on_despawn(player_entity, cleanup).unwrap();

            // add entry to tracker
            tracker.get_mut(&mut ui.rcommands).add(player_entity, placement_entry, score_entry, **score);

            // end style
            ui.style_stack.pop();
        }
    );

    // update score positions when the tracker is modified
    ui.rcommands.on(resource_mutation::<GameScoreboardTracker>(),
        |mut ui: UiUtils<MainUi>, tracker: ReactRes<GameScoreboardTracker>|
        {
            // iterate up from the lowest score
            let mut position = tracker.len();
            for score_widget in tracker.iter_score_entries()
            {
                if position == 0 { tracing::error!("game scoreboard tracker state is inconsisent"); return; }

                // edit the score position
                let Some(score_widget) = score_widget
                else { tracing::error!("scoreboard tracker has empty score entry"); continue; };

                let Some(score_layout) = ui.get_relative_layout_mut(&score_widget)
                else { tracing::error!("failed getting score widget's layout"); continue; };

                score_layout.relative_1.y = (position - 1) as f32 * ENTRY_HEIGHT;
                score_layout.relative_2.y = position       as f32 * ENTRY_HEIGHT;

                // the next score's position in the scoreboard
                position = position - 1;
            }
        }
    );
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameScoreboardPlugin(app: &mut App)
{
    app.insert_react_resource(GameScoreboardTracker::default());
}

//-------------------------------------------------------------------------------------------------------------------
