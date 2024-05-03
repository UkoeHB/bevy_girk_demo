use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn add_play_overlay(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (0., 50.), (0., 100.), add_lobby_display);
    ui.div_rel(area.end(""), (50., 100.), (0., 100.), add_lobby_list);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_section(ui: &mut UiBuilder<MainUi>, play_button: &Widget, menu_overlay: &Widget)
{
    let play_overlay = make_overlay(ui.tree(), menu_overlay, "play_overlay", false);

    ui.div_rel(play_button.end(""), (1.75, 97.25), (6.25, 90.5), |ui, area| {
        add_play_button(ui, area, &play_overlay)
    });
    ui.div(|ui| add_play_overlay(ui, &play_overlay));
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiPlaySectionPlugin(app: &mut App)
{
    app.add_plugins(UiPlayButtonPlugin)
        .add_plugins(UiLobbyDisplayPlugin)
        .add_plugins(UiLobbyListPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
