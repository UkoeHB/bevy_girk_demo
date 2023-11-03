//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_girk_utils::*;
use bevy_kot::ecs::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the make lobby window.
///
/// This is a reactive resource.
#[derive(Debug)]
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
            config      : ClickLobbyConfig{ max_players: 1, max_watchers: 0 },
            last_req    : None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn reset_window_state(mut rcommands: ReactCommands, mut window: ResMut<ReactRes<MakeLobbyWindow>>)
{
    *window.get_mut(&mut rcommands) = MakeLobbyWindow::default();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn send_make_lobby_request(
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    make_lobby    : Query<Entity, With<MakeLobby>>,
    window        : Res<ReactRes<MakeLobbyWindow>>
){
    //todo: handle single player vs multiplayer
    if window.is_single_player()
    { tracing::debug!("making local single player not yet supported, making game on server instead"); }

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
    else { return; };

    // save request
    let target_entity = make_lobby.single();
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_window_reactors(
    In(popup_pack) : In<BasicPopupPack>,
    mut ctx        : UiBuilderCtx,
    make_lobby     : Query<Entity, With<MakeLobby>>,
){
    let make_lobby_entity = make_lobby.single();

    // when a request completes
    let window_overlay = popup_pack.window_overlay;
    ctx.rcommands.add_entity_removal_reactor::<React<PendingRequest>>(
            make_lobby_entity,
            move |world: &mut World|
            {
                // access the window state
                let window = world.resource::<ReactRes<MakeLobbyWindow>>();
                let Some(req) = &window.last_req else { return; };
                let req_status = req.status();

                // do nothing if still sending
                if  (req_status == bevy_simplenet::RequestStatus::Sending) ||
                    (req_status == bevy_simplenet::RequestStatus::Waiting)
                { return; }

                // handle request result
                if req_status == bevy_simplenet::RequestStatus::Responded
                {
                    // close window
                    syscall(world, (MainUI, [], [window_overlay.clone()]), toggle_ui_visibility);

                    // reset window state
                    syscall(world, (), reset_window_state);
                }
                else
                {
                    // remove cached request
                    world.resource_mut::<ReactRes<MakeLobbyWindow>>().get_mut_noreact().last_req = None;                    
                }

            }
        );

    // prepare blocker for make button
    let accept_disable = make_overlay(ctx.ui(), &popup_pack.accept_button, "", false);
    let accept_entity = popup_pack.accept_entity;
    ctx.commands().spawn((accept_disable.clone(), UIInteractionBarrier::<MainUI>::default()));

    // disable 'make' button when disconnected and configs are non-local
    let make_button_disabler =
        move |world: &mut World|
        {
            let enable = **world.resource::<ReactRes<ConnectionStatus>>() == ConnectionStatus::Connected ||
                world.resource::<ReactRes<MakeLobbyWindow>>().is_single_player();
            syscall(world, (enable, accept_disable.clone(), accept_entity), toggle_button_availability);
        };

    ctx.rcommands.add_resource_mutation_reactor::<ConnectionStatus>(make_button_disabler.clone());
    ctx.rcommands.add_resource_mutation_reactor::<MakeLobbyWindow>(make_button_disabler);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // title
    //New Lobby

    // form section: 'join as' lobby member type for owner

    // form section: password

    // form section: max players, max watchers
    //disable options when disconnected (don't reset selections, just disable new choices)

    // indicator for local single-player
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// This is a reactive data event used to activate the window.
#[derive(Debug)]
pub(crate) struct ActivateMakeLobbyWindow;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_make_lobby_window(ctx: &mut UiBuilderCtx)
{
    // spawn window
    let popup_pack = spawn_basic_popup(ctx, (80., 80.), "Close", "Make", |_| (),
            |world| syscall(world, (), send_make_lobby_request),
        );

    // add window contents
    add_window_contents(ctx, &popup_pack.content_section);

    // open window when activation event is detected
    let window_overlay = popup_pack.window_overlay.clone();
    ctx.rcommands.add_event_reactor(
            move |world: &mut World, _event: ReactEvent<ActivateMakeLobbyWindow>|
            {
                syscall(world, (MainUI, [window_overlay.clone()], []), toggle_ui_visibility);
            }
        );

    // handle request results
    ctx.commands().add(move |world: &mut World| syscall(world, popup_pack, setup_window_reactors));

    // initialize ui
    ctx.rcommands.trigger_resource_mutation::<MakeLobbyWindow>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiMakeLobbyWindowPlugin(app: &mut App)
{
    app.insert_resource(ReactRes::new(MakeLobbyWindow::default()));
}

//-------------------------------------------------------------------------------------------------------------------
