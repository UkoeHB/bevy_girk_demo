//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::{*, builtin::*};
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the join lobby window.
///
/// This is a reactive resource.
#[derive(Debug)]
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

fn update_join_lobby_window(
    In(event)     : In<ReactEvent<ActivateJoinLobbyWindow>>,
    mut rcommands : ReactCommands,
    lobby_page    : Res<ReactRes<LobbyPage>>,
    mut window    : ResMut<ReactRes<JoinLobbyWindow>>,
){
    // get lobby id of lobby to join
    let lobby_index = event.get().lobby_list_index;

    let Some(lobby_contents) = lobby_page.get().get(lobby_index)
    else { tracing::error!(lobby_index, "failed accessing lobby contents for join lobby window"); return; };

    // update the window state
    *window.get_mut(&mut rcommands) = JoinLobbyWindow{ contents: Some(lobby_contents.clone()), ..Default::default() };
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn send_join_lobby_request(
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    join_lobby    : Query<Entity, (With<JoinLobby>, Without<React<PendingRequest>>)>,
    mut window    : ResMut<ReactRes<JoinLobbyWindow>>,
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
    else { return; };

    // save request
    let request = PendingRequest::new(new_req);
    rcommands.insert(target_entity, request.clone());
    window.get_mut_noreact().last_req = Some(request);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_window_reactors(
    In(popup_pack) : In<BasicPopupPack>,
    mut ui        : UiBuilder<MainUI>,
    join_lobby     : Query<Entity, With<JoinLobby>>,
){
    let join_lobby_entity = join_lobby.single();

    // when a request starts
    let accept_entity = popup_pack.accept_entity;
    ui.rcommands.on_entity_insertion::<React<PendingRequest>>(
            join_lobby_entity,
            move |world: &mut World|
            {
                // modify accept button text
                syscall(world, (accept_entity, String::from("...")), update_ui_text);
            }
        );

    // when a join-lobby request completes
    let window_overlay = popup_pack.window_overlay;
    ui.rcommands.on_entity_removal::<React<PendingRequest>>(
            join_lobby_entity,
            move |world: &mut World|
            {
                // access the window state
                let window = world.resource::<ReactRes<JoinLobbyWindow>>();
                let Some(req) = &window.last_req else { return; };
                let req_status = req.status();

                // do nothing if still sending
                if  (req_status == bevy_simplenet::RequestStatus::Sending) ||
                    (req_status == bevy_simplenet::RequestStatus::Waiting)
                { return; }

                // close window if request succeeded
                if req_status == bevy_simplenet::RequestStatus::Responded
                {
                    syscall(world, (MainUI, [], [window_overlay.clone()]), toggle_ui_visibility);
                }

                // remove cached request
                world.resource_mut::<ReactRes<JoinLobbyWindow>>().get_mut_noreact().last_req = None;

                // reset accept button text
                syscall(world, (accept_entity, String::from("Join")), update_ui_text);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // title
    //Join Lobby

    // info subtitle
    //lobby id: ###### -- owner id: ######
    //reacts to changes in window state

    // form section: lobby member type

    // form section: password
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
    let popup_pack = spawn_basic_popup(ui, (80., 80.), "Close", "Join", |_| (),
            |world| syscall(world, (), send_join_lobby_request),
        );

    // add window contents
    add_window_contents(ui, &popup_pack.content_section);

    // update window state and open window when activation event is detected
    let window_overlay = popup_pack.window_overlay.clone();
    ui.rcommands.on_event(
            move |world: &mut World, event: ReactEvent<ActivateJoinLobbyWindow>|
            {
                syscall(world, event, update_join_lobby_window);
                syscall(world, (MainUI, [window_overlay.clone()], []), toggle_ui_visibility);
            }
        );

    // setup window reactors
    ui.commands().add(move |world: &mut World| syscall(world, popup_pack, setup_window_reactors));

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<ReactRes<JoinLobbyWindow>>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiJoinLobbyWindowPlugin(app: &mut App)
{
    app.insert_resource(ReactRes::new(JoinLobbyWindow::default()));
}

//-------------------------------------------------------------------------------------------------------------------
