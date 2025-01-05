use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use client_core::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Debug)]
struct GameOverOverlay
{
    overlay: Widget,
}

//-------------------------------------------------------------------------------------------------------------------

fn activate_game_over_overlay(mut ui: UiUtils<MainUi>, overlay: Res<GameOverOverlay>)
{
    ui.toggle(true, &overlay.overlay);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_over(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // prep overlay
    let overlay = make_overlay(ui.tree(), area, "", false);
    overlay
        .fetch_mut(ui.tree())
        .unwrap()
        .get_container_mut()
        .set_render_depth(Modifier::Set(999.));

    // style
    let style = ui.style::<GameOverStyle>();

    // add screen
    let barrier = relative_widget(ui.tree(), overlay.end(""), (-10., 110.), (-10., 110.));
    let barrier_img = ImageElementBundle::new(
        &barrier,
        ImageParams::center()
            .with_width(Some(100.))
            .with_height(Some(100.))
            .with_color(style.background_color),
        ui.asset_server.load(style.background_img.0),
        style.background_img.1,
    );
    ui.commands().spawn(barrier_img);
    ui.commands()
        .spawn((barrier, UiInteractionBarrier::<MainUi>::default()));

    // add text
    let text_style = style.text.clone();
    ui.div_rel(overlay.end(""), (30., 70.), (40., 60.), move |ui, area| {
        // set text style
        ui.add_style(text_style);

        // spawn text
        spawn_basic_text(ui, area.clone(), TextParams::center(), "GAME OVER");
    });

    // insert overlay resource
    ui.commands().insert_resource(GameOverOverlay { overlay });
}

//-------------------------------------------------------------------------------------------------------------------

pub struct UiGameOverPlugin;

impl Plugin for UiGameOverPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(ClientMode::GameOver), activate_game_over_overlay);
    }
}

//-------------------------------------------------------------------------------------------------------------------
