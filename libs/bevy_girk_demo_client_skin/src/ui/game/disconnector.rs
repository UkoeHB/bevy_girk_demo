//local shortcuts
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use bevy_replicon::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_disconnector(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    spawn_basic_button(ui, &area, "Disconnect",
            |mut client: ResMut<RenetClient>|
            {
                client.disconnect();
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameDisconnectorPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
