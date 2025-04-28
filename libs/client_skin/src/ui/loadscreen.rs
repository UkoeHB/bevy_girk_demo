use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_game_fw::*;
use client_core::ClientLogicSet;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Default)]
struct RefreshLoadBar;

//-------------------------------------------------------------------------------------------------------------------

fn add_loadscreen(mut c: Commands, mut s: SceneBuilder)
{
    let scene = ("ui.skin", "loadscreen");
    c.ui_root().spawn_scene(scene, &mut s, |h| {
        h.despawn_on_broadcast::<ExitingInit>();
        h.insert(StateScoped(ClientAppState::Game));

        h.get("gutter::bar").update_on(
            broadcast::<RefreshLoadBar>(),
            |//
                    id: TargetId,
                    mut prev: Local<f32>,
                    mut c: Commands,
                    init_progress: Query<&GameInitProgress>,//
                |
                {
                    // Clamped to the previous value (since loading started) to avoid stutter.
                    let progress = init_progress
                        .single()
                        .map(|p| *p)
                        .unwrap_or(GameInitProgress::default())
                        .max(*prev)
                        .min(1.0);
                    //tracing::info!("progress({}): {progress}", init_progress.get_single().is_ok());
                    *prev = progress;
                    let mut ec = c.get_entity(*id)?;
                    ec.apply(Width(Val::Percent(progress * 100.0)));
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
        // We assume ClientAppState::Game is always delayed until LoadState::Done.
        app.add_systems(OnEnter(ClientAppState::Game), add_loadscreen)
            .add_systems(Update, broadcast_system::<RefreshLoadBar>.in_set(ClientLogicSet::End));
    }
}

//-------------------------------------------------------------------------------------------------------------------
