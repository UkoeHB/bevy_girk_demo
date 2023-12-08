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
    ui.div_rel(area.end(""), (0., 100.), ( 0.,  10.), |ui, area| add_game_header(ui, area));
    ui.div_rel(area.end(""), (0., 100.), (10.,  65.), |ui, area| add_game_scoreboard(ui, area));
    ui.div_rel(area.end(""), (0., 100.), (65., 100.), |ui, area| add_game_clicker(ui, area));
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
