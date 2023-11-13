//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_girk_utils::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;

//-------------------------------------------------------------------------------------------------------------------
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
        Self{
            member_type : ClickLobbyMemberType::Player,
            pwd         : String::default(),
            config      : ClickLobbyConfig{ max_players: 2, max_watchers: 1 },
            last_req    : None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn send_make_lobby_request(
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    make_lobby    : Query<Entity, (With<MakeLobby>, Without<React<PendingRequest>>)>,
    mut window    : ReactResMut<MakeLobbyWindow>
){
    // get request entity
    // - do nothing if there is already a pending request
    let Ok(target_entity) = make_lobby.get_single()
    else { tracing::warn!("ignoring join lobby request because a request is already pending"); return; };

    //todo: handle single player vs multiplayer
    if window.is_single_player()
    { tracing::warn!("making local single player not yet supported, making game on server instead"); }

    // request to make a lobby
    // - note: do not log the password
    tracing::trace!(?window.member_type, ?window.config, "requesting to make lobby");

    let Ok(new_req) = client.request(
            UserToHostRequest::MakeLobby{
                    mcolor : window.member_type.into(),
                    pwd    : window.pwd.clone(),
                    data   : ser_msg(&window.config)
                }
        )
    else { tracing::warn!("failed sending make lobby request to host server");  return; };

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
    make_lobby : Query<Entity, With<MakeLobby>>,
){
    let make_lobby_entity = make_lobby.single();

    // when a request starts
    let accept_entity = popup_pack.accept_entity;
    ui.rcommands.on(entity_insertion::<PendingRequest>(make_lobby_entity),
            move |mut text: TextHandle|
            {
                // modify accept button text
                text.write(accept_entity, 0, |text| write!(text, "{}", "...")).unwrap();
            }
        );

    // when a request completes
    let window_overlay = popup_pack.window_overlay;
    ui.rcommands.on(entity_removal::<PendingRequest>(make_lobby_entity),
            move |mut ui: UiUtils<MainUI>, mut window: ReactResMut<MakeLobbyWindow>|
            {
                // access the window state
                let Some(req) = &window.last_req else { return; };
                let req_status = req.status();

                // log error if still sending
                if  (req_status == bevy_simplenet::RequestStatus::Sending) ||
                    (req_status == bevy_simplenet::RequestStatus::Waiting)
                { tracing::error!("make lobby request terminated but window's cached request is still sending"); }

                // handle request result
                if req_status == bevy_simplenet::RequestStatus::Responded
                {
                    // close window
                    ui.toggle(false, &window_overlay);

                    // reset window state
                    *window.get_mut(&mut ui.builder.rcommands) = MakeLobbyWindow::default();
                }
                else
                {
                    // remove cached request, we must have failed
                    window.get_mut_noreact().last_req = None;                    
                }

                // reset accept button text
                ui.text.write(accept_entity, 0, |text| write!(text, "{}", accept_text)).unwrap();
            }
        );

    // prepare blocker for make button
    let accept_disable = make_overlay(ui.tree(), &popup_pack.accept_button, "", false);
    let accept_entity = popup_pack.accept_entity;
    ui.commands().spawn((accept_disable.clone(), UIInteractionBarrier::<MainUI>::default()));

    // disable 'make' button when disconnected and configs are non-local
    let make_button_disabler =
        move |mut ui: UiUtils<MainUI>, status: ReactRes<ConnectionStatus>, window: ReactRes<MakeLobbyWindow>|
        {
            let enable = (*status == ConnectionStatus::Connected) || window.is_single_player();
            ui.toggle_basic_button(enable, &accept_disable, accept_entity);
        };

    ui.rcommands.on(resource_mutation::<ConnectionStatus>(), make_button_disabler.clone());
    ui.rcommands.on(resource_mutation::<MakeLobbyWindow>(), make_button_disabler);
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
            "New Lobby"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_join_as_field(ui: &mut UiBuilder<MainUI>, area: &Widget)
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
            "Join as: Player\n(UI todo)"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_config_field(ui: &mut UiBuilder<MainUI>, area: &Widget)
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
            "Config: 1 player, 0 watchers\n(UI todo)"
        );
    //disable non-singleplayer options when disconnected (don't reset selections, just disable new choices)
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

fn add_connection_requirement_field(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // single-player text
    let sp_text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
            ui,
            sp_text.clone(),
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_height(Some(80.)),
            "Single-player lobby: does not require a server connection."
        );

    // multiplayer text
    let mp_text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
            ui,
            mp_text.clone(),
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_height(Some(80.)),
            "Multiplayer lobby: requires a server connection."
        );

    // adjust text depending on the lobby type
    let sp_widgets = [sp_text];
    let mp_widgets = [mp_text];
    ui.rcommands.on(resource_mutation::<MakeLobbyWindow>(),
            move |mut ui: UiUtils<MainUI>, window: ReactRes<MakeLobbyWindow>|
            {
                match window.is_single_player()
                {
                    true => ui.toggle_many(&sp_widgets, &mp_widgets),
                    false => ui.toggle_many(&mp_widgets, &sp_widgets),
                }
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // title
    let title_area = relative_widget(ui.tree(), area.end(""), (40., 60.), (5., 15.));
    ui.div(|ui| add_window_title(ui, &title_area));

    // form section: 'join as' lobby member type for owner
    let join_as_area = relative_widget(ui.tree(), area.end(""), (15., 47.), (20., 50.));
    ui.div(|ui| add_join_as_field(ui, &join_as_area));

    // form section: max players, max watchers
    let config_area = relative_widget(ui.tree(), area.end(""), (53., 85.), (20., 50.));
    ui.div(|ui| add_config_field(ui, &config_area));

    // form section: password
    let password_area = relative_widget(ui.tree(), area.end(""), (30., 70.), (57., 87.));
    ui.div(|ui| add_password_field(ui, &password_area));

    // indicator for local single-player
    let connection_requirement_area = relative_widget(ui.tree(), area.end(""), (10., 90.), (95., 100.));
    ui.div(|ui| add_connection_requirement_field(ui, &connection_requirement_area));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// This is a reactive data event used to activate the window.
#[derive(Debug)]
pub(crate) struct ActivateMakeLobbyWindow;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_make_lobby_window(ui: &mut UiBuilder<MainUI>)
{
    // spawn window
    let accept_text = "Make";
    let popup_pack = spawn_basic_popup(ui, "Close", accept_text, ||(), send_make_lobby_request);

    // add window contents
    ui.div(|ui| add_window_contents(ui, &popup_pack.content_section));

    // open window when activation event is detected
    let window_overlay = popup_pack.window_overlay.clone();
    ui.rcommands.on(event::<ActivateMakeLobbyWindow>(),
            move |mut ui: UiUtils<MainUI>|
            {
                ui.toggle(true, &window_overlay);
            }
        );

    // setup window reactors
    ui.commands().add(move |world: &mut World| syscall(world, (popup_pack, accept_text), setup_window_reactors));

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<MakeLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiMakeLobbyWindowPlugin(app: &mut App)
{
    app.insert_react_resource(MakeLobbyWindow::default())
        .add_react_event::<ActivateMakeLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------
