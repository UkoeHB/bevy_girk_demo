use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_demo_ui_prefab::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use bevy_replicon::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_disconnector(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    spawn_basic_button(ui, &area, "Disconnect", |mut client: ResMut<RenetClient>| {
        client.disconnect();
    });
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameDisconnectorPlugin(_app: &mut App) {}

//-------------------------------------------------------------------------------------------------------------------
