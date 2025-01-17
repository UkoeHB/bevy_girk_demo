use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use wiring_backend::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(super) fn build_lobby_list(h: &mut UiSceneHandle)
{
    h.get("loading_text").enable_if(
        (
            broadcast::<RequestStarted<LobbySearch>>(),
            broadcast::<RequestEnded<LobbySearch>>(),
        ),
        |_: TargetId, p: PendingRequestParam<LobbySearch>| p.has_request(),
    );
    // Note: this button doesn't use setup_request_tracker() because we show loading text separately.
    h.get("refresh_button").on_pressed(refresh_lobby_list);

    h.edit("page_view", |h| {
        h.update_on(
            resource_mutation::<LobbyPage>(),
            |id: TargetId, mut c: Commands, mut s: SceneBuilder, page: ReactRes<LobbyPage>| {
                // Clear current entries.
                c.get_entity(*id).result()?.despawn_descendants();

                // Spawn new entries
                for (idx, lobby) in page.get().iter().enumerate() {
                    c.ui_builder(*id)
                        .spawn_scene_and_edit(("ui.user", "lobby_list_entry"), &mut s, |h| {
                            h.get("text").update_text(format!(
                                        "Lobby: {:0>6}, Owner: {:0>6}, Players: {}/{}, Watchers: {}/{}",
                                        lobby.id % 1_000_000u64,
                                        lobby.owner_id % 1_000_000u128,
                                        lobby.num(ClickLobbyMemberType::Player),
                                        lobby.max(ClickLobbyMemberType::Player),
                                        lobby.num(ClickLobbyMemberType::Watcher),
                                        lobby.max(ClickLobbyMemberType::Watcher),
                                    ));
                            h.get("join_button").on_pressed(move |mut c: Commands| {
                                c.react()
                                    .broadcast(ActivateJoinLobbyPopup { lobby_list_index: idx });
                            });
                        });
                }

                DONE
            },
        );
    });

    h.get("page_stats::text").update_on(
        resource_mutation::<LobbyPage>(),
        |id: TargetId, mut e: TextEditor, page: ReactRes<LobbyPage>| {
            let (first, last, total) = page.stats();
            write_text!(e, *id, "({}-{} / {})", first, last, total);
        },
    );
    h.get("paginate_now_button")
        .on_pressed(request_lobby_list_now)
        .enable_if(
            resource_mutation::<LobbyPageRequest>(),
            |_: TargetId, last_req: ReactRes<LobbyPageRequest>| !last_req.is_now(),
        );
    h.get("paginate_left_button")
        .on_pressed(request_lobby_list_next_newer)
        .enable_if(
            resource_mutation::<LobbyPage>(),
            |_: TargetId, page: ReactRes<LobbyPage>| {
                let (first, _, _) = page.stats();
                first != 1
            },
        );
    h.get("paginate_right_button")
        .on_pressed(request_lobby_list_next_older)
        .enable_if(
            resource_mutation::<LobbyPage>(),
            |_: TargetId, page: ReactRes<LobbyPage>| {
                let (_, last, total) = page.stats();
                last != total
            },
        );
    h.get("paginate_oldest_button")
        .on_pressed(request_lobby_list_oldest)
        .enable_if(
            resource_mutation::<LobbyPageRequest>(),
            |_: TargetId, last_req: ReactRes<LobbyPageRequest>| !last_req.is_oldest(),
        );

    h.get("make_lobby_button")
        .on_pressed(|mut c: Commands| {
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
