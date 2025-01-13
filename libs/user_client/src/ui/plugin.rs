use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn build_ui(mut c: Commands, mut s: SceneBuilder)
{
    c.ui_root()
        .spawn_scene_and_edit(("ui.user", "main"), &mut s, |h| {
            h.insert(StateScoped(ClientAppState::Client));

            let content_id = h.get_entity("content")?;

            h.edit("sidebar", |h| build_sidebar(h, content_id));

            OK
        });
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            // TODO: is there a better way to do this?
            .add_systems(
                OnEnter(LoadState::Done),
                build_ui.run_if(in_state(ClientAppState::Client)),
            )
            .add_systems(
                OnEnter(ClientAppState::Client),
                build_ui.run_if(in_state(LoadState::Done)),
            )
            // ui plugins
            .add_plugins(MenuContentPlugin)
            .add_plugins(UiConnectionStatusPlugin)
            .add_plugins(UiPlaySectionPlugin)
            .add_plugins(UiAckLobbyWindowPlugin)
            .add_plugins(UiGameInProgressPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
