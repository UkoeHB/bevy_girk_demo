//local shortcuts
use crate::*;

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

fn send_lobby_nack(
    mut rcommands   : ReactCommands,
    client          : Res<HostUserClient>,
    mut ack_request : ReactResMut<AckRequestData>,
){
    // fail if nack was already sent
    if ack_request.is_nacked() { tracing::error!("ignoring duplicate lobby nack"); return; };

    // send lobby nack
    let Some(lobby_id) = ack_request.get()
    else { tracing::warn!("tried to nack lobby but there is no ack request"); return; };
    tracing::trace!(lobby_id, "nacking lobby");

    let Ok(_) = client.send(UserToHostMsg::NackPendingLobby{ id: lobby_id })
    else { tracing::warn!("failed sending nack lobby message to host server"); return; };

    // save action
    ack_request.get_mut(&mut rcommands).set_nacked();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn send_lobby_ack(
    mut rcommands   : ReactCommands,
    client          : Res<HostUserClient>,
    mut ack_request : ReactResMut<AckRequestData>,
){
    // fail if ack was already sent
    if ack_request.is_acked() { tracing::error!("ignoring duplicate lobby ack"); return; };

    // send lobby ack
    let Some(lobby_id) = ack_request.get()
    else { tracing::warn!("tried to ack lobby but there is no ack request"); return; };
    tracing::trace!(lobby_id, "acking lobby");

    let Ok(_) = client.send(UserToHostMsg::AckPendingLobby{ id: lobby_id })
    else { tracing::warn!("failed sending ack lobby message to host server"); return; };

    // save action
    ack_request.get_mut(&mut rcommands).set_acked();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_title(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // title text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
            ui,
            text,
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_height(Some(70.)),
            "Start Game"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_timer(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // add text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    let text_entity = spawn_basic_text(
            ui,
            text,
            TextParams::center()
                .with_depth(700.)  //todo: remove when lunex is fixed
                .with_height(Some(60.)),
            "99"
        );

    // update the text when the ack request changes
    ui.rcommands.on(resource_mutation::<AckRequestData>(),
            move |mut text: TextHandle, ack_request: ReactRes<AckRequestData>|
            {
                let time_remaining_secs = ack_request.time_remaining_for_display().as_secs();
                text.write(text_entity, 0, |text| write!(text, "{}", time_remaining_secs)).unwrap();
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // title
    let title_area = relative_widget(ui.tree(), area.end(""), (40., 60.), (20., 40.));
    ui.div(|ui| add_window_title(ui, &title_area));

    // timer
    let timer_area = relative_widget(ui.tree(), area.end(""), (40., 60.), (57., 87.));
    ui.div(|ui| add_timer(ui, &timer_area));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_ack_lobby_window(ui: &mut UiBuilder<MainUi>)
{
    // spawn window
    ui.edit_style::<BasicPopup>(
            |popup_style|
            {
                popup_style.proportions       = Vec2{ x: 70., y: 50. };
                popup_style.content_percent   = 65.;
                popup_style.button_ratio      = 0.85;
                popup_style.button_gap        = 5.;
                popup_style.button_dead_space = popup_style.button_dead_space + 5.;
            }
        ).unwrap();

    let popup_pack = spawn_basic_popup(ui, "Reject", "Accept", send_lobby_nack, send_lobby_ack);

    // add window contents
    ui.div(|ui| add_window_contents(ui, &popup_pack.content_section));

    // disabler for reject button
    let reject_disable_overlay = make_overlay(ui.tree(), &popup_pack.cancel_button, "", true);
    ui.commands().spawn((reject_disable_overlay.clone(), UIInteractionBarrier::<MainUi>::default()));

    // disabler for accept button
    let enable_disable_overlay = make_overlay(ui.tree(), &popup_pack.accept_button, "", true);
    ui.commands().spawn((enable_disable_overlay.clone(), UIInteractionBarrier::<MainUi>::default()));

    // setup window reactor
    let window_overlay = popup_pack.window_overlay.clone();
    let reject_button_entity = popup_pack.cancel_entity;
    let accept_button_entity = popup_pack.accept_entity;
    ui.rcommands.on(resource_mutation::<AckRequestData>(),
            move |mut ui: UiUtils<MainUi>, ack_request: ReactRes<AckRequestData>|
            {
                // open/close window based on if the ack request is set
                ui.toggle(ack_request.is_set(), &window_overlay);

                // enable the reject button when nack was not sent
                let enable_reject = !ack_request.is_nacked();
                ui.toggle_basic_button(enable_reject, &reject_disable_overlay, reject_button_entity);

                // enable the accept button when nack and ack were not sent
                let enable_accept = !ack_request.is_nacked() && !ack_request.is_acked();
                ui.toggle_basic_button(enable_accept, &enable_disable_overlay, accept_button_entity);
            }
        );

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<AckRequestData>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiAckLobbyWindowPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
