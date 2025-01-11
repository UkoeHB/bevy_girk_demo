use std::fmt::Write;

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot_ui::builtin::MainUi;
use bevy_kot_ui::{relative_widget, UiBuilder};
use bevy_lunex::prelude::*;
use ui_prefab::*;
use wiring_backend::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn setup_window_reactors(
    In((popup_pack, accept_text)): In<(BasicPopupPack, &'static str)>,
    mut ui: UiBuilder<MainUi>,
    join_lobby: Query<Entity, With<JoinLobby>>,
)
{
    let join_lobby_entity = join_lobby.single();

    // when a request starts
    let accept_entity = popup_pack.accept_entity;
    ui.commands().react().on(
        entity_insertion::<PendingRequest>(join_lobby_entity),
        move |mut text: TextEditor| {
            // modify accept button text
            text.write(accept_entity, |text| write!(text, "{}", "..."));
        },
    );

    // when a join-lobby request completes
    let window_overlay = popup_pack.window_overlay;
    ui.commands().react().on(
        entity_removal::<PendingRequest>(join_lobby_entity),
        move |mut ui: UiUtils<MainUi>, mut window: ReactResMut<JoinLobbyWindow>| {
            // access the window state
            let Some(req) = &window.last_req else {
                return;
            };
            let req_status = req.status();

            // log error if still sending
            if (req_status == bevy_simplenet::RequestStatus::Sending)
                || (req_status == bevy_simplenet::RequestStatus::Waiting)
            {
                tracing::error!("join lobby request terminated but window's cached request is still sending");
            }

            // close window if request succeeded
            if req_status == bevy_simplenet::RequestStatus::Responded {
                ui.toggle(false, &window_overlay);
            }

            // remove cached request
            window.get_noreact().last_req = None;

            // reset accept button text
            ui.text
                .write(accept_entity, |text| write!(text, "{}", accept_text));
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_window_title(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // title text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(100.)), "Join Lobby");
}

//-------------------------------------------------------------------------------------------------------------------

fn add_subtitle(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // add text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));

    let default_text = "Lobby: ?????? -- Owner: ??????";
    let text_entity = spawn_basic_text(ui, text, TextParams::center().with_height(Some(40.)), default_text);

    // update the text when the window changes
    ui.commands().react().on(
        resource_mutation::<JoinLobbyWindow>(),
        move |mut text: TextEditor, window: ReactRes<JoinLobbyWindow>| {
            if let Some(lobby_contents) = &window.contents {
                let id = lobby_contents.id % 1_000_000u64;
                let owner_id = lobby_contents.owner_id % 1_000_000u128;
                text.write(text_entity, |text| {
                    write!(text, "Lobby: {:0>6} -- Owner: {:0>6}", id, owner_id)
                });
            } else {
                text.write(text_entity, |text| write!(text, "{}", default_text));
            }
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_member_type_field(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // field outline
    spawn_plain_outline(ui, area.clone());

    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
        ui,
        text,
        TextParams::center().with_width(Some(75.)),
        "Member type: Player\n(UI todo)",
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_password_field(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // field outline
    spawn_plain_outline(ui, area.clone());

    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
        ui,
        text,
        TextParams::center().with_width(Some(75.)),
        "Password: <empty>\n(UI todo)",
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (40., 60.), (5., 15.), add_window_title);
    ui.div_rel(area.end(""), (10., 90.), (20., 30.), add_subtitle);
    ui.div_rel(area.end(""), (15., 47.), (40., 70.), add_member_type_field);
    ui.div_rel(area.end(""), (53., 85.), (40., 70.), add_password_field);
}

//-------------------------------------------------------------------------------------------------------------------

/// This is a reactive data event used to activate the window.
#[derive(Debug, Clone)]
pub(crate) struct ActivateJoinLobbyWindow
{
    /// Index in the lobby list of the lobby corresponding to this lobby window.
    ///
    /// Only valid in the tick where it is set.
    pub(crate) lobby_list_index: usize,
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_join_lobby_window(ui: &mut UiBuilder<MainUi>)
{
    // spawn window
    let accept_text = "Join";
    let popup_pack = spawn_basic_popup(ui, "Close", "Join", || (), send_join_lobby_request);

    // add window contents
    ui.div(|ui| add_window_contents(ui, &popup_pack.content_section));

    // update window state and open window when activation event is detected
    let window_overlay = popup_pack.window_overlay.clone();
    ui.commands().react().on(
        broadcast::<ActivateJoinLobbyWindow>(),
        move |events: BroadcastEvent<ActivateJoinLobbyWindow>,
              mut ui: UiUtils<MainUi>,
              lobby_page: ReactRes<LobbyPage>,
              mut window: ReactResMut<JoinLobbyWindow>| {
            // get lobby id of lobby to join
            let Some(event) = events.read() else {
                return;
            };
            let lobby_index = event.lobby_list_index;

            let Some(lobby_contents) = lobby_page.get().get(lobby_index) else {
                tracing::error!(lobby_index, "failed accessing lobby contents for join lobby window");
                return;
            };

            // update the window state
            *window.get_mut(&mut ui.builder.commands()) =
                JoinLobbyWindow { contents: Some(lobby_contents.clone()), ..Default::default() };

            // open the window
            ui.toggle(true, &window_overlay);
        },
    );

    // setup window reactors
    ui.commands()
        .add(move |world: &mut World| world.syscall((popup_pack, accept_text), setup_window_reactors));

    // initialize ui
    ui.commands()
        .react()
        .trigger_resource_mutation::<JoinLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiJoinLobbyWindowPlugin;

impl Plugin for UiJoinLobbyWindowPlugin
{
    fn build(&self, _app: &mut App) {}
}

//-------------------------------------------------------------------------------------------------------------------
