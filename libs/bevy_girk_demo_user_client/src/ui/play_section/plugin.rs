//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_fn_plugin::*;
use bevy_kot::prelude::{*, builtin::*};
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_play_overlay(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // lobby display area (left half)
    let lobby_display_area = relative_widget(ui.tree(), area.end(""), (0., 50.), (0., 100.));
    add_lobby_display(ui, &lobby_display_area);

    // lobby list area (right half)
    let lobby_list_area = relative_widget(ui.tree(), area.end(""), (50., 100.), (0., 100.));
    add_lobby_list(ui, &lobby_list_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_section(ui: &mut UiBuilder<MainUI>, play_button: Widget, menu_overlay: Widget)
{
    let play_button_area = relative_widget(ui.tree(), play_button.end(""), (1.75, 97.25), (6.25, 90.5));
    let play_overlay = make_overlay(ui.tree(), &menu_overlay, "play_overlay", false);
    add_play_button(ui, &play_button_area, &play_overlay);
    add_play_overlay(ui, &play_overlay);
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiPlaySectionPlugin(app: &mut App)
{
    app.add_plugins(UiPlayButtonPlugin)
        .add_plugins(UiLobbyDisplayPlugin)
        .add_plugins(UiLobbyListPlugin)
        ;
}

//-------------------------------------------------------------------------------------------------------------------
