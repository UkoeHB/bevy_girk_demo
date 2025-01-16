use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_cobweb_ui::sickle::PseudoState;
use bevy_girk_backend_public::HostUserClient;
use smol_str::SmolStr;

use super::*;
use crate::*;

//-------------------------------------------------------------------------------------------------------------------

const STATUS_CONNECTED_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("Connected"));
const STATUS_CONNECTING_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("Connecting"));
const STATUS_DEAD_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("Dead"));

const IN_LOBBY_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("InLobby"));

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn build_sidebar(h: &mut UiSceneHandle, content_id: Entity)
{
    // menu options
    h.get("options")
        .spawn_scene_and_edit(("ui.user", "menu_button"), |h| {
            h.get("text").update_text("Home");
            h.on_select(|mut c: Commands, mut s: SceneBuilder| {
                c.get_entity(content_id)?.despawn_descendants();

                c.ui_builder(content_id).spawn_scene_and_edit(
                    ("ui.user", "home_section"),
                    &mut s,
                    build_home_section,
                );

                DONE
            });

            // Start with the home section selected.
            let id = h.id();
            h.react().entity_event(id, Select);
        })
        .spawn_scene_and_edit(("ui.user", "menu_button"), |h| {
            h.get("text").update_on(
                resource_mutation::<LobbyDisplay>(),
                |id: TargetId, mut e: TextEditor, display: ReactRes<LobbyDisplay>| match display.is_set() {
                    true => {
                        write_text!(e, *id, "In Lobby");
                    }
                    false => {
                        write_text!(e, *id, "Play");
                    }
                },
            );
            h.on_select(|mut c: Commands, mut s: SceneBuilder| {
                c.get_entity(content_id)?.despawn_descendants();

                c.ui_builder(content_id).spawn_scene_and_edit(
                    ("ui.user", "play_section"),
                    &mut s,
                    build_play_section,
                );

                DONE
            });
            h.update_on(
                resource_mutation::<LobbyDisplay>(),
                |id: TargetId, mut c: Commands, ps: PseudoStateParam, display: ReactRes<LobbyDisplay>| {
                    match display.is_set() {
                        true => {
                            ps.try_insert(&mut c, *id, IN_LOBBY_PSEUDOSTATE.clone());
                        }
                        false => {
                            ps.try_remove(&mut c, *id, IN_LOBBY_PSEUDOSTATE.clone());
                        }
                    }
                },
            );
        })
        .spawn_scene_and_edit(("ui.user", "menu_button"), |h| {
            h.get("text").update_text("Settings");
            h.on_select(|mut c: Commands, mut s: SceneBuilder| {
                c.get_entity(content_id)?.despawn_descendants();

                c.ui_builder(content_id).spawn_scene_and_edit(
                    ("ui.user", "settings_section"),
                    &mut s,
                    build_settings_section,
                );

                DONE
            });
        });

    // footer
    h.get("footer")
        .spawn_scene_and_edit(("ui.user", "user_info"), |h| {
            h.get("client_id").update_on(
                broadcast::<NewHostUserClient>(),
                |id: TargetId, client: Res<HostUserClient>, mut e: TextEditor| {
                    write_text!(e, *id, "ID: {}", client.id());
                },
            );
            h.get("connection_status").update_on(
                resource_mutation::<ConnectionStatus>(),
                |//
                    id: TargetId,
                    mut c: Commands,
                    mut e: TextEditor,
                    ps: PseudoStateParam,
                    status: ReactRes<ConnectionStatus>,//
                | {
                    ps.try_remove(*id, &mut c, STATUS_CONNECTED_PSEUDOSTATE.clone());
                    ps.try_remove(*id, &mut c, STATUS_CONNECTING_PSEUDOSTATE.clone());
                    ps.try_remove(*id, &mut c, STATUS_DEAD_PSEUDOSTATE.clone());
                    match *status {
                        ConnectionStatus::Connected => {
                            write_text!(e, *id, "Connected");
                            ps.try_insert(*id, &mut c, STATUS_CONNECTED_PSEUDOSTATE.clone());
                        }
                        ConnectionStatus::Connecting => {
                            write_text!(e, *id, "Connecting...");
                            ps.try_insert(*id, &mut c, STATUS_CONNECTING_PSEUDOSTATE.clone());
                        }
                        ConnectionStatus::Dead => {
                            write_text!(e, *id, "Dead");
                            ps.try_insert(*id, &mut c, STATUS_DEAD_PSEUDOSTATE.clone());
                        }
                    }
                },
            );
        });
}

//-------------------------------------------------------------------------------------------------------------------

/// Resource that tracks which 'primary' section of the menu is visible.
//TODO: update this when changing sections
#[derive(Resource, Debug, Default)]
pub(crate) enum MenuContentSection
{
    #[default]
    Home,
    Play,
    Settings,
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct UiSidebarPlugin;

impl Plugin for UiSidebarPlugin
{
    fn build(&self, app: &mut App)
    {
        app.init_resource::<MenuContentSection>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
