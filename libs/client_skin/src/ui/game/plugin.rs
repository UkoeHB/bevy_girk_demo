use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn add_game_header<'a>(l: &mut LoadedScene<'a, UiBuilder<'a, Entity>>)
{
    let scene = ("ui.skin.game", "header");
    l.load_scene_and_edit(scene, |l| {
        l.get("fps_text").update_on(
            resource_mutation::<FpsTracker>(),
            |id: UpdateId, mut e: TextEditor, fps: ReactRes<FpsTracker>| {
                // only refresh once per second
                if fps.current_time().as_secs() <= fps.previous_time().as_secs() {
                    return;
                }

                write_text!(e, *id, "FPS: {}", fps.fps());
            },
        );
    });
}

//-------------------------------------------------------------------------------------------------------------------

/// Clicker. This is how you 'play' the demo.
fn add_clicker<'a>(l: &mut LoadedScene<'a, UiBuilder<'a, Entity>>)
{
    l.load_scene_and_edit(("ui.skin.game", "click_button"), |l| {
        l.on_pressed(|player_input: Res<Sender<PlayerInput>>| {
            let _ = player_input.send(PlayerInput::ClickButton);
        });
    });
}

//-------------------------------------------------------------------------------------------------------------------

/// Disconnect button. Lets you test in-game disconnects.
fn add_disconnector<'a>(l: &mut LoadedScene<'a, UiBuilder<'a, Entity>>)
{
    l.load_scene_and_edit(("ui.skin.game", "disconnect_button"), |l| {
        l.on_pressed(|mut client: ResMut<RenetClient>| {
            client.disconnect();
        });
    });
}

//-------------------------------------------------------------------------------------------------------------------

fn build_ui(mut c: Commands, mut s: ResMut<SceneLoader>)
{
    let scene = ("ui.skin.game", "game");
    c.ui_root().load_scene_and_edit(scene, &mut s, |l| {
        l.insert(StateScoped(ClientInstanceState::Game));

        add_game_header(l);
        add_game_scoreboard(l);
        add_clicker(l);
        add_disconnector(l);
    });
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(ScoreboardPlugin)
            .add_systems(OnEnter(ClientState::Prep), build_ui.after(RefreshScoreboardSet));
    }
}

//-------------------------------------------------------------------------------------------------------------------
