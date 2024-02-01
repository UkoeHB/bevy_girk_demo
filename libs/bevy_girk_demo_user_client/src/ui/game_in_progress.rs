//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_girk_user_client_utils::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn reconnect_game(
    mut rcommands    : ReactCommands,
    client           : Res<HostUserClient>,
    monitor          : ReactRes<ClientMonitor>,
    starter          : ReactRes<ClientStarter>,
    reconnect_button : Query<Entity, (With<ReconnectorButton>, Without<React<PendingRequest>>)>,
){
    // check for existing request
    let Ok(target_entity) = reconnect_button.get_single()
    else { tracing::error!("ignoring reconnect game request because a request is already pending"); return };

    // sanity checks
    if monitor.is_running() { tracing::error!("reconnect game selected but client is currently in a game"); }
    if !starter.has_starter() { tracing::error!("reconnect game selected but client cannot reconnect"); return; }
    let Some(game_id) = starter.game_id()
    else { tracing::error!("starter game id missing on reconnect request"); return; };

    // request new connect token
    let new_req= client.request(UserToHostRequest::GetConnectToken{ id: game_id });

    // save request
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_background(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // style
    let style = ui.style::<GameInProgressStyle>();

    // add screen
    let barrier_img = ImageElementBundle::new(
            area,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.background_color),
            ui.asset_server.load(style.background_img.0),
            style.background_img.1
        );
    ui.commands().spawn((barrier_img, UiInteractionBarrier::<MainUi>::default()));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_text(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(ui.style::<GameInProgressStyle>().text.clone());

    // spawn text
    spawn_basic_text(ui, area.clone(), TextParams::center(), "Game In Progress");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_reconnect_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(ui.style::<GameInProgressStyle>().reconnect_button.clone());

    // spawn button
    let default_text = "Reconnect";
    let button_entity = spawn_basic_button(ui, area, default_text, reconnect_game);

    // enable button when we can reconnect
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);
    ui.rcommands.on((resource_mutation::<ClientMonitor>(), resource_mutation::<ClientStarter>()),
            move |mut ui: UiUtils<MainUi>, monitor: ReactRes<ClientMonitor>, starter: ReactRes<ClientStarter>|
            {
                ui.builder.style_stack.push();
                ui.builder.add_style(ui.builder.style::<GameInProgressStyle>().reconnect_button_avail.clone());

                let enable = !monitor.is_running() && starter.has_starter();
                ui.toggle_basic_button(enable, button_entity, &disable_overlay);

                ui.builder.style_stack.pop();
            }
        );

    // show pending text when waiting for response
    ui.commands().add(
            move |world: &mut World|
            {
                syscall(world, (button_entity, default_text), setup_simple_pending_button_text::<ReconnectorButton>);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_in_progress(ui: &mut UiBuilder<MainUi>)
{
    // overlay
    let window_overlay = make_overlay(ui.tree(), &Widget::new("root"), "", false);
    window_overlay.fetch_mut(ui.tree()).unwrap().get_container_mut().set_render_depth(Modifier::Set(750.));

    // contents
    ui.div_rel(window_overlay.end(""), (-5000., 5000.), (-5000., 5000.), add_background);
    ui.div_rel(window_overlay.end(""), (40., 60.), (45., 55.), add_text);
    ui.div_rel(window_overlay.end(""), (40., 60.), (60., 70.), add_reconnect_button);

    // show overlay when a game is in progress
    ui.rcommands.on((resource_mutation::<ClientMonitor>(), resource_mutation::<ClientStarter>()),
            move |mut ui: UiUtils<MainUi>, monitor: ReactRes<ClientMonitor>, starter: ReactRes<ClientStarter>|
            {
                let enable = monitor.is_running() || starter.has_starter();
                ui.toggle(enable, &window_overlay);
            }
        );

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<ClientMonitor>();
    ui.rcommands.trigger_resource_mutation::<ClientStarter>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameInProgressPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
