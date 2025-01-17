use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use super::*;
use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn build_play_section(h: &mut UiSceneHandle)
{
    // TODO: find a better abstraction for managing page navigation ??
    h.update_on(
        resource_mutation::<LobbyDisplay>(),
        |id: TargetId, mut c: Commands, mut s: SceneBuilder, display: ReactRes<LobbyDisplay>| {
            c.get_entity(*id).result()?.despawn_descendants();
            match display.is_set() {
                true => {
                    c.ui_builder(*id).spawn_scene_and_edit(
                        ("ui.user", "lobby_display"),
                        &mut s,
                        build_lobby_display,
                    );
                }
                false => {
                    c.ui_builder(*id)
                        .spawn_scene_and_edit(("ui.user", "lobby_list"), &mut s, build_lobby_list);
                }
            }
            DONE
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiPlaySectionPlugin;

impl Plugin for UiPlaySectionPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(UiLobbyDisplayPlugin)
            .add_plugins(UiLobbyListPlugin)
            .add_plugins(UiJoinLobbyPopupPlugin)
            .add_plugins(UiMakeLobbyPopupPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
