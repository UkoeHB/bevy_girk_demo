use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use super::*;
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
        app.add_systems(OnEnter(ClientAppState::Client), build_ui)
            // ui plugins
            .add_plugins(UiSidebarPlugin)
            .add_plugins(UiReconnectingPlugin)
            .add_plugins(UiAckLobbyPopupPlugin)
            // ui menu sections
            .add_plugins(UiHomeSectionPlugin)
            .add_plugins(UiPlaySectionPlugin)
            .add_plugins(UiSettingsSectionPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
