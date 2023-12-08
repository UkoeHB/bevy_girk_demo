//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (0., 100.), ( 0.,  10.), add_game_header);
    ui.div_rel(area.end(""), (0., 100.), (10.,  65.), add_game_scoreboard);
    ui.div_rel(area.end(""), (0., 100.), (65., 100.), add_game_clicker);
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGamePlugin(app: &mut App)
{
    app.add_plugins(UiGameHeaderPlugin)
        .add_plugins(UiGameScoreboardPlugin)
        .add_plugins(UiGameClickerPlugin);
}

//-------------------------------------------------------------------------------------------------------------------
