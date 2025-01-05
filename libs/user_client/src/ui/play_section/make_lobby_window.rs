use std::fmt::Write;

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_utils::*;
use bevy_kot_ui::builtin::*;
use bevy_kot_ui::{make_overlay, relative_widget, UiBuilder};
use bevy_lunex::prelude::*;
use ui_prefab::*;
use wiring_backend::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the make lobby window.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
struct MakeLobbyWindow
{
    /// Cached member type.
    member_type: ClickLobbyMemberType,
    /// Cached password.
    pwd: String,
    /// Cached lobby config.
    config: ClickLobbyConfig,

    /// Last request sent
    last_req: Option<PendingRequest>,
}

impl MakeLobbyWindow
{
    fn is_single_player(&self) -> bool
    {
        self.config.is_single_player()
    }
}

impl Default for MakeLobbyWindow
{
    fn default() -> Self
    {
        Self {
            member_type: ClickLobbyMemberType::Player,
            pwd: String::default(),
            config: ClickLobbyConfig { max_players: 1, max_watchers: 0 },
            last_req: None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Convert window state to lobby contents.
///
/// Panics if not single-player.
fn single_player_lobby(owner_id: u128, window: &MakeLobbyWindow) -> ClickLobbyContents
{
    if !window.is_single_player() {
        panic!("cannot convert make lobby window to lobby contents for multiplayer lobbies");
    }

    ClickLobbyContents {
        id: 0u64,
        owner_id,
        config: window.config.clone(),
        players: vec![(bevy_simplenet::env_type(), owner_id)],
        watchers: vec![],
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
struct MadeLocalLobby;

//-------------------------------------------------------------------------------------------------------------------

fn make_local_lobby(
    mut c: Commands,
    client: Res<HostUserClient>,
    make_lobby: Query<(), (With<MakeLobby>, With<React<PendingRequest>>)>,
    mut lobby_display: ReactResMut<LobbyDisplay>,
    window: ReactRes<MakeLobbyWindow>,
)
{
    // do nothing if there is a pending request
    if !make_lobby.is_empty() {
        tracing::warn!("ignoring make local lobby request because a multiplayer request is pending");
        return;
    };

    // do nothing if we are in a hosted lobby
    if lobby_display.is_hosted() {
        tracing::warn!("ignoring make local lobby request because we are in a multiplayer lobby");
        return;
    };

    // make a local lobby
    // - note: do not log the password
    tracing::trace!(?window.member_type, ?window.config, "making a local lobby");
    lobby_display
        .get_mut(&mut c)
        .set(single_player_lobby(client.id(), &window), LobbyType::Local);

    // send event for UI updates
    c.react().broadcast(MadeLocalLobby);
}

//-------------------------------------------------------------------------------------------------------------------

fn send_make_lobby_request(
    mut c: Commands,
    client: Res<HostUserClient>,
    make_lobby: Query<Entity, (With<MakeLobby>, Without<React<PendingRequest>>)>,
    mut window: ReactResMut<MakeLobbyWindow>,
)
{
    // get request entity
    // - do nothing if there is already a pending request
    let Ok(target_entity) = make_lobby.get_single() else {
        tracing::warn!("ignoring make lobby request because a request is already pending");
        return;
    };

    // request to make a lobby
    // - note: do not log the password
    tracing::trace!(?window.member_type, ?window.config, "requesting to make lobby");

    let new_req = client.request(UserToHostRequest::MakeLobby {
        mcolor: window.member_type.into(),
        pwd: window.pwd.clone(),
        data: ser_msg(&window.config),
    });

    // save request
    let request = PendingRequest::new(new_req);
    c.react().insert(target_entity, request.clone());
    window.get_noreact().last_req = Some(request);
}
//-------------------------------------------------------------------------------------------------------------------

fn make_a_lobby(world: &mut World)
{
    match world.react_resource::<MakeLobbyWindow>().is_single_player() {
        true => syscall(world, (), make_local_lobby),
        false => syscall(world, (), send_make_lobby_request),
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn setup_window_reactors(
    In((popup_pack, accept_text)): In<(BasicPopupPack, &'static str)>,
    mut ui: UiBuilder<MainUi>,
    make_lobby: Query<Entity, With<MakeLobby>>,
)
{
    let make_lobby_entity = make_lobby.single();

    // when activation event is detected
    let window_overlay = popup_pack.window_overlay.clone();
    ui.commands().react().on(
        broadcast::<ActivateMakeLobbyWindow>(),
        move |mut ui: UiUtils<MainUi>| {
            // open window
            ui.toggle(true, &window_overlay);
        },
    );

    // when a local lobby is made
    let window_overlay = popup_pack.window_overlay.clone();
    ui.commands()
        .react()
        .on(broadcast::<MadeLocalLobby>(), move |mut ui: UiUtils<MainUi>| {
            // close window
            ui.toggle(false, &window_overlay);
        });

    // when a make lobby request starts
    let accept_entity = popup_pack.accept_entity;
    ui.commands().react().on(
        entity_insertion::<PendingRequest>(make_lobby_entity),
        move |mut text: TextEditor| {
            // modify accept button text
            text.write(accept_entity, |text| write!(text, "{}", "..."));
        },
    );

    // when a make lobby request completes
    let window_overlay = popup_pack.window_overlay;
    ui.commands().react().on(
        entity_removal::<PendingRequest>(make_lobby_entity),
        move |mut ui: UiUtils<MainUi>, mut window: ReactResMut<MakeLobbyWindow>| {
            // access the window state
            let Some(req) = &window.last_req else {
                return;
            };
            let req_status = req.status();

            // log error if still sending
            if (req_status == bevy_simplenet::RequestStatus::Sending)
                || (req_status == bevy_simplenet::RequestStatus::Waiting)
            {
                tracing::error!("make lobby request terminated but window's cached request is still sending");
            }

            // handle request result
            if req_status == bevy_simplenet::RequestStatus::Responded {
                // close window
                ui.toggle(false, &window_overlay);
            } else {
                // remove cached request, we must have failed
                window.get_noreact().last_req = None;
            }

            // reset accept button text
            ui.text
                .write(accept_entity, |text| write!(text, "{}", accept_text));
        },
    );

    // prepare blocker for make button
    let accept_entity = popup_pack.accept_entity;
    let accept_disable = spawn_basic_button_blocker(&mut ui, &popup_pack.accept_button, false);

    // disable 'make' button
    // - when disconnected and configs are non-local
    // - when in a hosted lobby or waiting for a hosted lobby request
    ui.commands().react().on(
        (
            resource_mutation::<ConnectionStatus>(),
            resource_mutation::<MakeLobbyWindow>(),
            resource_mutation::<LobbyDisplay>(),
        ),
        move |mut ui: UiUtils<MainUi>,
              status: ReactRes<ConnectionStatus>,
              window: ReactRes<MakeLobbyWindow>,
              make_lobby: Query<(), (With<MakeLobby>, With<React<PendingRequest>>)>,
              lobby_display: ReactResMut<LobbyDisplay>| {
            let enable = (*status == ConnectionStatus::Connected) || window.is_single_player();
            let enable = enable && make_lobby.is_empty() && !lobby_display.is_hosted();
            ui.toggle_basic_button(enable, accept_entity, &accept_disable);
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_window_title(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // title text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(100.)), "New Lobby");
}

//-------------------------------------------------------------------------------------------------------------------

fn add_join_as_field(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // field outline
    spawn_plain_outline(ui, area.clone());

    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
        ui,
        text,
        TextParams::center().with_width(Some(75.)),
        "Join as: Player\n(UI todo)",
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_config_field(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // field outline
    spawn_plain_outline(ui, area.clone());

    let singleplayer_overlay = make_overlay(ui.tree(), area, "", true);
    spawn_basic_text(
        ui,
        singleplayer_overlay.clone(),
        TextParams::center().with_width(Some(75.)),
        "Config: 1 player, 0 watchers\n(UI todo)",
    );

    let multiplayer_overlay = make_overlay(ui.tree(), area, "", false);
    spawn_basic_text(
        ui,
        multiplayer_overlay.clone(),
        TextParams::center().with_width(Some(75.)),
        "Config: 2 players, 1 watcher\n(UI todo)",
    );

    // basic toggle for single/multiplayer
    //todo: replace with proper UI
    let button = relative_widget(ui.tree(), area.end(""), (40., 60.), (75., 95.));
    spawn_basic_button(
        ui,
        &button,
        "Toggle",
        move |mut toggle: Local<bool>, mut ui: UiUtils<MainUi>, mut window: ReactResMut<MakeLobbyWindow>| {
            ui.toggle(*toggle, &singleplayer_overlay);
            ui.toggle(!*toggle, &multiplayer_overlay);

            match *toggle {
                true => {
                    window.get_mut(&mut ui.builder.commands()).config =
                        ClickLobbyConfig { max_players: 1, max_watchers: 0 };
                }
                false => {
                    window.get_mut(&mut ui.builder.commands()).config =
                        ClickLobbyConfig { max_players: 2, max_watchers: 1 };
                }
            }

            *toggle = !*toggle;
        },
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

fn add_connection_requirement_field(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // single-player text
    let sp_text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
        ui,
        sp_text.clone(),
        TextParams::center().with_height(Some(80.)),
        "Single-player lobby: does not require a server connection.",
    );

    // multiplayer text
    let mp_text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
        ui,
        mp_text.clone(),
        TextParams::center().with_height(Some(80.)),
        "Multiplayer lobby: requires a server connection.",
    );

    // adjust text depending on the lobby type
    let sp_widgets = [sp_text];
    let mp_widgets = [mp_text];
    ui.commands().react().on(
        resource_mutation::<MakeLobbyWindow>(),
        move |mut ui: UiUtils<MainUi>, window: ReactRes<MakeLobbyWindow>| match window.is_single_player() {
            true => ui.toggle_many(&sp_widgets, &mp_widgets),
            false => ui.toggle_many(&mp_widgets, &sp_widgets),
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (40., 60.), (5., 15.), add_window_title);
    ui.div_rel(area.end(""), (15., 47.), (20., 50.), add_join_as_field);
    ui.div_rel(area.end(""), (53., 85.), (20., 50.), add_config_field);
    ui.div_rel(area.end(""), (30., 70.), (57., 87.), add_password_field);
    ui.div_rel(area.end(""), (10., 90.), (95., 100.), add_connection_requirement_field);
}

//-------------------------------------------------------------------------------------------------------------------

/// This is a reactive data event used to activate the window.
#[derive(Debug)]
pub(crate) struct ActivateMakeLobbyWindow;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_make_lobby_window(ui: &mut UiBuilder<MainUi>)
{
    // spawn window
    let accept_text = "Make";
    let popup_pack = spawn_basic_popup(ui, "Close", accept_text, || (), make_a_lobby);

    // add window contents
    ui.div(|ui| add_window_contents(ui, &popup_pack.content_section));

    // setup window reactors
    ui.commands()
        .add(move |world: &mut World| syscall(world, (popup_pack, accept_text), setup_window_reactors));

    // initialize ui
    ui.commands()
        .react()
        .trigger_resource_mutation::<MakeLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiMakeLobbyWindowPlugin;

impl Plugin for UiMakeLobbyWindowPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_react_resource(MakeLobbyWindow::default());
    }
}

//-------------------------------------------------------------------------------------------------------------------
