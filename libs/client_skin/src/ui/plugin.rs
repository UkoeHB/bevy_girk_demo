use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use super::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(CobwebUiPlugin)
            .load("client_skin/main.cob")
            .add_plugins(LoadScreenPlugin)
            .add_plugins(GameUiPlugin)
            .add_plugins(GameOverPlugin)
            //.add_plugins(UiDebugOverlayPlugin)  //DEBUG ONLY
            ;
    }
}

//-------------------------------------------------------------------------------------------------------------------
