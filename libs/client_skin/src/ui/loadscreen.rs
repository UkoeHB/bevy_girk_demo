use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_game_fw::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Default)]
struct RefreshLoadBar;

//-------------------------------------------------------------------------------------------------------------------

fn add_loadscreen(mut c: Commands, mut s: ResMut<SceneBuilder>)
{
    let scene = ("ui.skin", "loadscreen");
    c.ui_root().spawn_scene_and_edit(scene, &mut s, |h| {
        h.despawn_on_broadcast::<ExitingInit>();
        h.insert(StateScoped(ClientInstanceState::Game));

        h.get("gutter::bar").update_on(
            broadcast::<RefreshLoadBar>(),
            |//
                    id: UpdateId,
                    mut prev: Local<f32>,
                    mut c: Commands,
                    progress: Query<&GameInitProgress>,//
                |
                {
                    // Clamped to the previous value (since loading started) to avoid stutter.
                    let progress = progress
                        .get_single()
                        .map(|p| *p)
                        .unwrap_or(GameInitProgress::default())
                        .max(*prev)
                        .min(1.0);
                    *prev = progress;
                    let mut ec = c.get_entity(*id).result()?;
                    ec.apply(Width(Val::Px(progress)));
                    DONE
                },
        );
    });
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct LoadScreenPlugin;

impl Plugin for LoadScreenPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_reactor(broadcast::<EnteringConnecting>(), add_loadscreen)
            .add_systems(Update, broadcast_system::<RefreshLoadBar>.in_set(ClientLogicSet::End));
    }
}

//-------------------------------------------------------------------------------------------------------------------
