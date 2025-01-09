use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn edit_header(mut h: UiSceneHandle)
{
    h.get("fps::text").update_on(
        resource_mutation::<FpsTracker>(),
        |id: UpdateId, mut e: TextEditor, fps: ReactRes<FpsTracker>| {
            // only refresh once per second
            if fps.current_time().as_secs() <= fps.previous_time().as_secs() {
                return;
            }

            write_text!(e, *id, "FPS: {}", fps.fps());
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn edit_content(mut h: UiSceneHandle)
{
    edit_scoreboard(h.get("scoreboard"));

    // Clicker. This is how you 'play' the demo.
    h.get("button_area::click_button")
        .on_pressed(|player_input: Res<Sender<PlayerInput>>| {
            let _ = player_input.send(PlayerInput::ClickButton);
        });
}

//-------------------------------------------------------------------------------------------------------------------

fn edit_footer(mut h: UiSceneHandle)
{
    // Disconnect button. Lets you test in-game disconnects.
    h.get("disconnect_button")
        .on_pressed(|mut client: ResMut<RenetClient>| {
            client.disconnect();
        });
}

//-------------------------------------------------------------------------------------------------------------------

fn build_ui(mut c: Commands, mut s: ResMut<SceneLoader>)
{
    let scene = ("ui.skin.game", "game");
    c.ui_root().load_scene_and_edit(scene, &mut s, |h| {
        h.insert(StateScoped(ClientInstanceState::Game));

        edit_header(h.get("header"));
        edit_content(h.get("content"));
        edit_footer(h.get("footer"));
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
