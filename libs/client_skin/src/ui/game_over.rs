use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::ClientAppState;
use client_core::*;

//-------------------------------------------------------------------------------------------------------------------

fn game_over_screen(mut c: Commands, mut s: SceneBuilder)
{
    let scene = ("ui.skin", "gameover");
    c.ui_root().spawn_scene(scene, &mut s, |h| {
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
