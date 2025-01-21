use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_backend_public::*;
use wiring_backend::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn build_lobby_display(h: &mut UiSceneHandle)
{
    h.get("header::lobby_info::text").update_on(
        resource_mutation::<LobbyDisplay>(),
        |id: TargetId, mut e: TextEditor, display: ReactRes<LobbyDisplay>| {
            let lobby_contents = display.get().result()?;
            let lobby_id = lobby_contents.id % 1_000_000u64;
            let owner_id = lobby_contents.owner_id % 1_000_000u128;
            write_text!(e, *id, "Lobby: {:0>6} -- Owner: {:0>6}", lobby_id, owner_id);
            OK
        },
    );
    h.get("header::member_count::players::text").update_on(
        resource_mutation::<LobbyDisplay>(),
        |id: TargetId, mut e: TextEditor, display: ReactRes<LobbyDisplay>| {
            let lobby_contents = display.get().result()?;
            let num_members = lobby_contents.num(ClickLobbyMemberType::Player);
            let max_members = lobby_contents.max(ClickLobbyMemberType::Player);
            write_text!(e, *id, "Players: {}/{}", num_members, max_members);
            OK
        },
    );
    h.get("header::member_count::watchers::text").update_on(
        resource_mutation::<LobbyDisplay>(),
        |id: TargetId, mut e: TextEditor, display: ReactRes<LobbyDisplay>| {
            let lobby_contents = display.get().result()?;
            let num_members = lobby_contents.num(ClickLobbyMemberType::Watcher);
            let max_members = lobby_contents.max(ClickLobbyMemberType::Watcher);
            write_text!(e, *id, "Watchers: {}/{}", num_members, max_members);
            OK
        },
    );

    h.get("content::member_list::view::shim").update_on(
        resource_mutation::<LobbyDisplay>(),
        |id: TargetId, mut c: Commands, mut s: SceneBuilder, display: ReactRes<LobbyDisplay>| {
            // clean up previous members list
            c.get_entity(*id).result()?.despawn_descendants();

            let lobby_content = display.get().result()?;
            for (_, player_id) in lobby_content.players.iter() {
                c.ui_builder(*id).spawn_scene_and_edit(
                    ("ui.user.sections.play", "lobby_display_member"),
                    &mut s,
                    |h| {
                        h.get("text")
                            .update_text(format!("Player: {:0>6}", player_id % 1_000_000u128));
                    },
                );
            }
            for (_, watcher_id) in lobby_content.watchers.iter() {
                c.ui_builder(*id).spawn_scene_and_edit(
                    ("ui.user.sections.play", "lobby_display_member"),
                    &mut s,
                    |h| {
                        h.get("text")
                            .update_text(format!("Watcher: {:0>6}", watcher_id % 1_000_000u128));
                    },
                );
            }

            DONE
        },
    );

    h.edit("footer::leave::button", |h| {
        setup_request_tracker::<LeaveLobby>(h);
        h.enable_if(
            resource_mutation::<LobbyDisplay>(),
            |_: TargetId, display: ReactRes<LobbyDisplay>| display.is_set(),
        )
        .on_pressed(leave_current_lobby);
    });
    h.edit("footer::start_button", |h| {
        setup_request_tracker::<LaunchLobby>(h);
        h.enable_if(
            resource_mutation::<LobbyDisplay>(),
            |_: TargetId, display: ReactRes<LobbyDisplay>, client: Res<HostUserClient>| match display.get() {
                Some(data) => {
                    let owns = data.owner_id == client.id();
                    let single_player = display.is_local();
                    let can_launch_hosted = data.can_launch_hosted();

                    owns && (single_player || can_launch_hosted)
                }
                None => false,
            },
        )
        .on_pressed(start_current_lobby);
    });
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiLobbyDisplayPlugin;

impl Plugin for UiLobbyDisplayPlugin
{
    fn build(&self, _app: &mut App) {}
}

//-------------------------------------------------------------------------------------------------------------------
