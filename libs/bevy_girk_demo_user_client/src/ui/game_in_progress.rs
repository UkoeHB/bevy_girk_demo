//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn reconnect_game(
    mut rcommands    : ReactCommands,
    client           : Res<HostUserClient>,
    game_monitor     : ReactRes<GameMonitor>,
    reconnector      : ReactRes<GameReconnector>,
    reconnect_button : Query<Entity, (With<ReconnectorButton>, Without<React<PendingRequest>>)>,
){
    // check for existing request
    let Ok(target_entity) = reconnect_button.get_single()
    else { tracing::error!("ignoring reconnect game request because a request is already pending"); return };

    // sanity checks
    if game_monitor.is_running() { tracing::error!("reconnect game selected but client is currently in a game"); }
    if !reconnector.can_reconnect() { tracing::error!("reconnect game selected but client cannot reconnect"); return; }
    let Some(game_id) = reconnector.game_id()
    else { tracing::error!("reconnector game id missing on reconnect request"); return; };

    // request new connect token
    let Ok(new_req) = client.request(UserToHostRequest::GetConnectToken{ id: game_id })
    else { tracing::warn!(game_id, "failed sending get connect token request to host server"); return; };

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
    let button_entity = spawn_basic_button(ui, area, "Reconnect", reconnect_game);

    // enable button when we can reconnect
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);
    ui.rcommands.on((resource_mutation::<GameMonitor>(), resource_mutation::<GameReconnector>()),
            move |mut ui: UiUtils<MainUi>, game_monitor: ReactRes<GameMonitor>, reconnector: ReactRes<GameReconnector>|
            {
                ui.builder.style_stack.push();
                ui.builder.add_style(ui.builder.style::<GameInProgressStyle>().reconnect_button_avail.clone());

                let enable = !game_monitor.is_running() && reconnector.can_reconnect();
                ui.toggle_basic_button(enable, button_entity, &disable_overlay);

                ui.builder.style_stack.pop();
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
    ui.rcommands.on((resource_mutation::<GameMonitor>(), resource_mutation::<GameReconnector>()),
            move |mut ui: UiUtils<MainUi>, game_monitor: ReactRes<GameMonitor>, reconnector: ReactRes<GameReconnector>|
            {
                let enable = game_monitor.is_running() || reconnector.can_reconnect();
                ui.toggle(enable, &window_overlay);
            }
        );

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<GameMonitor>();
    ui.rcommands.trigger_resource_mutation::<GameReconnector>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameInProgressPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
