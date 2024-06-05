use bevy::prelude::*;
use bevy_girk_demo_ui_prefab::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use bevy_renet2::renet2::RenetClient;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_disconnector(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    spawn_basic_button(ui, &area, "Disconnect", |mut client: ResMut<RenetClient>| {
        client.disconnect();
    });
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiGameDisconnectorPlugin;

impl Plugin for UiGameDisconnectorPlugin
{
    fn build(&self, _app: &mut App) {}
}

//-------------------------------------------------------------------------------------------------------------------
