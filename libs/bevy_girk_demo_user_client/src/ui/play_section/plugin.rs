//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::ui::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_play_overlay(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // lobby display area (left half)
    let lobby_display_area = relative_widget(ctx, area.end(""), (0., 50.), (0., 100.));
    add_lobby_display(ctx, &lobby_display_area);

    // lobby list area (right half)
    let lobby_list_area = relative_widget(ctx, area.end(""), (50., 100.), (0., 100.));
    add_lobby_list(ctx, &lobby_list_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_play_section(ctx: &mut UiBuilderCtx, play_button: Widget, menu_overlay: Widget)
{
    let play_button_area = relative_widget(ctx, play_button.end(""), (1.75, 97.25), (6.25, 91.));
    let play_overlay = make_overlay(ctx.ui, &menu_overlay, "play_overlay", false);
    add_play_button(ctx, &play_button_area, &play_overlay);
    add_play_overlay(ctx, &play_overlay);
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
