//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the join lobby window.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
struct JoinLobbyWindow
{
    /// Lobby contents.
    contents: Option<ClickLobbyContents>,

    /// Cached member type.
    member_type: ClickLobbyMemberType,
    /// Cached password.
    pwd: String,

    /// Last request sent
    last_req: Option<PendingRequest>,
}

impl Default for JoinLobbyWindow
{
    fn default() -> Self
    {
        Self{
            contents    : None,
            member_type : ClickLobbyMemberType::Player,
            pwd         : String::default(),
            last_req    : None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn send_join_lobby_request(
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    join_lobby    : Query<Entity, (With<JoinLobby>, Without<React<PendingRequest>>)>,
    mut window    : ReactResMut<JoinLobbyWindow>,
){
    // get request entity
    // - do nothing if there is already a pending request
    let Ok(target_entity) = join_lobby.get_single()
    else { tracing::warn!("ignoring join lobby request because a request is already pending"); return; };

    // fail if there is no lobby
    let Some(lobby_contents) = &window.contents
    else { tracing::error!("lobby contents are missing for join lobby request"); return; };

    // request to join the specified lobby
    // - note: do not log the password
    let lobby_id = lobby_contents.id;
    tracing::trace!(lobby_id, ?window.member_type, "requesting to join lobby");

    let Ok(new_req) = client.request(
            UserToHostRequest::JoinLobby{ id: lobby_id, mcolor: window.member_type.into(), pwd: window.pwd.clone() }
        )
    else { tracing::warn!("failed sending join lobby request to host server"); return; };

    // save request
    let request = PendingRequest::new(new_req);
    rcommands.insert(target_entity, request.clone());
    window.get_mut_noreact().last_req = Some(request);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_window_reactors(
    In((
        popup_pack,
        accept_text,
    ))         : In<(BasicPopupPack, &'static str)>,
    mut ui     : UiBuilder<MainUI>,
    join_lobby : Query<Entity, With<JoinLobby>>,
){
    let join_lobby_entity = join_lobby.single();

    // when a request starts
    let accept_entity = popup_pack.accept_entity;
    ui.rcommands.on(entity_insertion::<PendingRequest>(join_lobby_entity),
            move |mut text: TextHandle|
            {
                // modify accept button text
                text.write(accept_entity, 0, |text| write!(text, "{}", "...")).unwrap();
            }
        );

    // when a join-lobby request completes
    let window_overlay = popup_pack.window_overlay;
    ui.rcommands.on(entity_removal::<PendingRequest>(join_lobby_entity),
            move |mut ui: UiUtils<MainUI>, mut window: ReactResMut<JoinLobbyWindow>|
            {
                // access the window state
                let Some(req) = &window.last_req else { return; };
                let req_status = req.status();

                // log error if still sending
                if  (req_status == bevy_simplenet::RequestStatus::Sending) ||
                    (req_status == bevy_simplenet::RequestStatus::Waiting)
                { tracing::error!("join lobby request terminated but window's cached request is still sending"); }

                // close window if request succeeded
                if req_status == bevy_simplenet::RequestStatus::Responded
                {
                    ui.toggle(false, &window_overlay);
                }

                // remove cached request
                window.get_mut_noreact().last_req = None;

                // reset accept button text
                ui.text.write(accept_entity, 0, |text| write!(text, "{}", accept_text)).unwrap();
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_title(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // title text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
            ui,
            text,
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_height(Some(100.)),
            "Join Lobby"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_subtitle(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // add text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));

    let default_text = "Lobby: ?????? -- Owner: ??????";
    let text_entity = spawn_basic_text(
            ui,
            text,
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_height(Some(40.)),
            default_text
        );

    // update the text when the window changes
    ui.rcommands.on(resource_mutation::<JoinLobbyWindow>(),
            move |mut text: TextHandle, window: ReactRes<JoinLobbyWindow>|
            {
                if let Some(lobby_contents) = &window.contents
                {
                    let id       = lobby_contents.id % 1_000_000u64;
                    let owner_id = lobby_contents.owner_id % 1_000_000u128;
                    text.write(text_entity, 0, |text| write!(text, "Lobby: {} -- Owner: {}", id, owner_id)).unwrap();
                }
                else
                {
                    text.write(text_entity, 0, |text| write!(text, "{}", default_text)).unwrap();
                }
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_member_type_field(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // field outline
    spawn_plain_outline(ui, area.clone(), Some(700.));

    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
            ui,
            text,
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_width(Some(75.)),
            "Member type: Player\n(UI todo)"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_password_field(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // field outline
    spawn_plain_outline(ui, area.clone(), Some(700.));

    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
            ui,
            text,
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_width(Some(75.)),
            "Password: <empty>\n(UI todo)"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // title
    let title_area = relative_widget(ui.tree(), area.end(""), (40., 60.), (5., 15.));
    ui.div(|ui| add_window_title(ui, &title_area));

    // info subtitle
    let subtitle_area = relative_widget(ui.tree(), area.end(""), (10., 90.), (20., 30.));
    ui.div(|ui| add_subtitle(ui, &subtitle_area));

    // form section: lobby member type
    let member_type_area = relative_widget(ui.tree(), area.end(""), (15., 47.), (40., 70.));
    ui.div(|ui| add_member_type_field(ui, &member_type_area));

    // form section: password
    let password_area = relative_widget(ui.tree(), area.end(""), (53., 85.), (40., 70.));
    ui.div(|ui| add_password_field(ui, &password_area));
}

//-------------------------------------------------------------------------------------------------------------------
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

pub(crate) fn add_join_lobby_window(ui: &mut UiBuilder<MainUI>)
{
    // spawn window
    let accept_text = "Join";
    let popup_pack = spawn_basic_popup(ui, "Close", "Join", ||(), send_join_lobby_request);

    // add window contents
    ui.div(|ui| add_window_contents(ui, &popup_pack.content_section));

    // update window state and open window when activation event is detected
    let window_overlay = popup_pack.window_overlay.clone();
    ui.rcommands.on(event::<ActivateJoinLobbyWindow>(),
            move
            |
                mut events : ReactEvents<ActivateJoinLobbyWindow>,
                mut ui     : UiUtils<MainUI>,
                lobby_page : ReactRes<LobbyPage>,
                mut window : ReactResMut<JoinLobbyWindow>
            |
            {
                // get lobby id of lobby to join
                let Some(event) = events.next() else { return; };
                let lobby_index = event.lobby_list_index;

                let Some(lobby_contents) = lobby_page.get().get(lobby_index)
                else { tracing::error!(lobby_index, "failed accessing lobby contents for join lobby window"); return; };

                // update the window state
                *window.get_mut(&mut ui.builder.rcommands) = JoinLobbyWindow{
                        contents: Some(lobby_contents.clone()),
                        ..Default::default()
                    };

                // open the window
                ui.toggle(true, &window_overlay);
            }
        );

    // setup window reactors
    ui.commands().add(move |world: &mut World| syscall(world, (popup_pack, accept_text), setup_window_reactors));

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<JoinLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiJoinLobbyWindowPlugin(app: &mut App)
{
    app.insert_react_resource(JoinLobbyWindow::default())
        .add_react_event::<ActivateJoinLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------
