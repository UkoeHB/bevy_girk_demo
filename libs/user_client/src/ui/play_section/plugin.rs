use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use super::*;
use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn build_play_section(h: &mut UiSceneHandle)
{
    // TODO: find a better abstraction for managing page navigation ??
    h.update_on(
        resource_mutation::<LobbyDisplay>(),
        |id: TargetId, mut c: Commands, mut s: SceneBuilder, display: ReactRes<LobbyDisplay>| {
            c.get_entity(*id)?.despawn_descendants();
            match display.is_set() {
                true => {
                    c.ui_builder(*id)
                        .spawn_scene_and_edit(("ui.user", "lobby_display"), build_lobby_display);
                }
                false => {
                    c.ui_builder(*id)
                        .spawn_scene_and_edit(("ui.user", "lobby_list"), build_lobby_list);
                }
            }
            DONE
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct UiPlaySectionPlugin;

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
