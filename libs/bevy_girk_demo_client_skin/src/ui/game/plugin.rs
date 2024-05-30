use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (0., 100.), (0., 10.), add_game_header);
    ui.div_rel(area.end(""), (0., 100.), (10., 65.), add_game_scoreboard);
    ui.div_rel(area.end(""), (0., 100.), (65., 100.), add_game_clicker);
    ui.div_rel(area.end(""), (3., 13.), (90., 95.), add_disconnector);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiGamePlugin;

impl Plugin for UiGamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(UiGameHeaderPlugin)
            .add_plugins(UiGameScoreboardPlugin)
            .add_plugins(UiGameClickerPlugin)
            .add_plugins(UiGameDisconnectorPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
