use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use client_core::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn game_over_screen(mut c: Commands, mut s: SceneBuilder)
{
    let scene = ("ui.skin", "gameover");
    c.ui_root().spawn_scene_and_edit(scene, &mut s, |h| {
        h.insert(StateScoped(ClientAppState::Game));
    });
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct GameOverPlugin;

impl Plugin for GameOverPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(ClientState::GameOver), game_over_screen);
    }
}

//-------------------------------------------------------------------------------------------------------------------