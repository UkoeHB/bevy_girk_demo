use std::fmt::Write;

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot_ui::builtin::MainUi;
use bevy_kot_ui::{relative_widget, UiBuilder};
use bevy_lunex::prelude::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn add_window_title(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // title text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    spawn_basic_text(
        ui,
        text,
        TextParams::center()
            .with_depth(700.) //todo: remove when lunex is fixed
            .with_height(Some(70.)),
        "Start Game",
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_timer(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // add text
    let text = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 100.));
    let text_entity = spawn_basic_text(
        ui,
        text,
        TextParams::center()
            .with_depth(700.) //todo: remove when lunex is fixed
            .with_height(Some(60.)),
        "99",
    );

    // update the text when the ack request changes
    ui.commands().react().on(
        resource_mutation::<AckRequestData>(),
        move |mut text: TextEditor, ack_request: ReactRes<AckRequestData>| {
            let time_remaining_secs = ack_request.time_remaining_for_display().as_secs();
            text.write(text_entity, |text| write!(text, "{}", time_remaining_secs));
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_window_contents(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (40., 60.), (20., 40.), add_window_title);
    ui.div_rel(area.end(""), (40., 60.), (57., 87.), add_timer);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_ack_lobby_window(ui: &mut UiBuilder<MainUi>)
{
    // spawn window
    ui.edit_style::<BasicPopup>(|popup_style| {
        popup_style.proportions = Vec2 { x: 70., y: 50. };
        popup_style.content_percent = 65.;
        popup_style.button_ratio = 0.85;
        popup_style.button_gap = 5.;
        popup_style.button_dead_space = popup_style.button_dead_space + 5.;
    })
    .unwrap();

    let popup_pack = spawn_basic_popup(ui, "Reject", "Accept", send_lobby_nack, send_lobby_ack);

    // add window contents
    ui.div(|ui| add_window_contents(ui, &popup_pack.content_section));

    // disabler for reject button
    let reject_disable_overlay = spawn_basic_button_blocker(ui, &popup_pack.cancel_button, true);

    // disabler for accept button
    let enable_disable_overlay = spawn_basic_button_blocker(ui, &popup_pack.accept_button, true);

    // setup window reactor
    let window_overlay = popup_pack.window_overlay.clone();
    let reject_button_entity = popup_pack.cancel_entity;
    let accept_button_entity = popup_pack.accept_entity;
    ui.commands().react().on(
        resource_mutation::<AckRequestData>(),
        move |mut ui: UiUtils<MainUi>, ack_request: ReactRes<AckRequestData>| {
            // open/close window based on if the ack request is set
            ui.toggle(ack_request.is_set(), &window_overlay);

            // enable the reject button when nack was not sent
            let enable_reject = !ack_request.is_nacked();
            ui.toggle_basic_button(enable_reject, reject_button_entity, &reject_disable_overlay);

            // enable the accept button when nack and ack were not sent
            let enable_accept = !ack_request.is_nacked() && !ack_request.is_acked();
            ui.toggle_basic_button(enable_accept, accept_button_entity, &enable_disable_overlay);
        },
    );

    // initialize ui
    ui.commands()
        .react()
        .trigger_resource_mutation::<AckRequestData>();
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiAckLobbyWindowPlugin;

impl Plugin for UiAckLobbyWindowPlugin
{
    fn build(&self, _app: &mut App) {}
}

//-------------------------------------------------------------------------------------------------------------------
