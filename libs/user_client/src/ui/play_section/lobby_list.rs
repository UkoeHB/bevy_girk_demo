use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use wiring_backend::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn build_lobby_list(h: &mut UiSceneHandle)
{
    h.get("content::upper_control::loading_text").enable_if(
        (
            broadcast::<RequestStarted<LobbySearch>>(),
            broadcast::<RequestEnded<LobbySearch>>(),
        ),
        |_: TargetId, p: PendingRequestParam<LobbySearch>| p.has_request(),
    );
    // Note: this button doesn't use setup_request_tracker() because we show loading text separately.
    h.get("content::upper_control::refresh_button")
        .on_pressed(refresh_lobby_list);

    h.edit("content::list::view::shim::entries", |h| {
        h.update_on(
            resource_mutation::<LobbyPage>(),
            |id: TargetId, mut c: Commands, mut s: SceneBuilder, page: ReactRes<LobbyPage>| {
                // Clear current entries.
                c.get_entity(*id).result()?.despawn_descendants();

                // Spawn new entries
                for (idx, lobby) in page.get().iter().enumerate() {
                    c.ui_builder(*id).spawn_scene(
                        ("ui.user.sections.play", "lobby_list_entry_lobby"),
                        &mut s,
                        |h| {
                            h.get("text")
                                .update_text(format!("{:0>6}", lobby.id % 1_000_000u64));
                        },
                    );
                    c.ui_builder(*id).spawn_scene(
                        ("ui.user.sections.play", "lobby_list_entry_owner"),
                        &mut s,
                        |h| {
                            h.get("text")
                                .update_text(format!("{:0>6}", lobby.owner_id % 1_000_000u128));
                        },
                    );
                    c.ui_builder(*id).spawn_scene(
                        ("ui.user.sections.play", "lobby_list_entry_players"),
                        &mut s,
                        |h| {
                            h.get("text").update_text(format!(
                                "{}/{}",
                                lobby.num(ClickLobbyMemberType::Player),
                                lobby.max(ClickLobbyMemberType::Player)
                            ));
                        },
                    );
                    c.ui_builder(*id).spawn_scene(
                        ("ui.user.sections.play", "lobby_list_entry_watchers"),
                        &mut s,
                        |h| {
                            h.get("text").update_text(format!(
                                "{}/{}",
                                lobby.num(ClickLobbyMemberType::Watcher),
                                lobby.max(ClickLobbyMemberType::Watcher)
                            ));
                        },
                    );
                    c.ui_builder(*id).spawn_scene(
                        ("ui.user.sections.play", "lobby_list_entry_join_button"),
                        &mut s,
                        |h| {
                            h.on_pressed(move |mut c: Commands| {
                                c.react()
                                    .broadcast(ActivateJoinLobbyPopup { lobby_list_index: idx });
                            });
                        },
                    );
                }

                DONE
            },
        );
    });

    h.get("content::controls::page_stats::text").update_on(
        resource_mutation::<LobbyPage>(),
        |id: TargetId, mut e: TextEditor, page: ReactRes<LobbyPage>| {
            let (first, last, total) = page.stats();
            write_text!(e, *id, "({}-{} / {})", first, last, total);
        },
    );
    h.get("content::controls::paginate_now_button")
        .on_pressed(request_lobby_list_now)
        .enable_if(
            resource_mutation::<LobbyPageRequest>(),
            |_: TargetId, last_req: ReactRes<LobbyPageRequest>| !last_req.is_now(),
        );
    h.get("content::controls::paginate_left_button")
        .on_pressed(request_lobby_list_next_newer)
        .enable_if(
            resource_mutation::<LobbyPage>(),
            |_: TargetId, page: ReactRes<LobbyPage>| {
                let (first, _, _) = page.stats();
                first != 1
            },
        );
    h.get("content::controls::paginate_right_button")
        .on_pressed(request_lobby_list_next_older)
        .enable_if(
            resource_mutation::<LobbyPage>(),
            |_: TargetId, page: ReactRes<LobbyPage>| {
                let (_, last, total) = page.stats();
                last != total
            },
        );
    h.get("content::controls::paginate_oldest_button")
        .on_pressed(request_lobby_list_oldest)
        .enable_if(
            resource_mutation::<LobbyPageRequest>(),
            |_: TargetId, last_req: ReactRes<LobbyPageRequest>| !last_req.is_oldest(),
        );

    h.get("content::make_lobby_button")
        .on_pressed(|mut c: Commands| {
            tracing::trace!("activating make lobby popup");
            c.react().broadcast(ActivateMakeLobbyPopup);
        })
        // Disable when lobby display is set to avoid race conditions around auto-leaving the current lobby
        // when making a new lobby.
        .enable_if(
            resource_mutation::<LobbyDisplay>(),
            |_: TargetId, display: ReactRes<LobbyDisplay>| !display.is_set(),
        );
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiLobbyListPlugin;

impl Plugin for UiLobbyListPlugin
{
    fn build(&self, _app: &mut App) {}
}

//-------------------------------------------------------------------------------------------------------------------
