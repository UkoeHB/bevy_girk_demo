use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use client_core::*;
use game_core::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_clicker(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (40., 60.), (10., 40.), |ui, area| {
        ui.add_style(ui.style::<GameClickerStyle>().button.clone());

        spawn_basic_button(ui, &area, "CLICK ME", |player_input: Res<Sender<PlayerClientInput>>| {
            let _ = player_input.send(PlayerClientInput::Play(PlayerInput::ClickButton));
        });
    });
}

//-------------------------------------------------------------------------------------------------------------------

pub struct UiGameClickerPlugin;

impl Plugin for UiGameClickerPlugin
{
    fn build(&self, _app: &mut App) {}
}

//-------------------------------------------------------------------------------------------------------------------
