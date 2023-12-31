//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_clicker(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (40., 60.), (10., 40.), |ui, area| {
        ui.add_style(ui.style::<GameClickerStyle>().button.clone());

        spawn_basic_button(ui, &area, "CLICK ME",
            |player_input: Res<Sender<PlayerClientInput>>|
            {
                let _ = player_input.send(PlayerClientInput::Play(PlayerInput::ClickButton));
            }
        );
    });
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameClickerPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
