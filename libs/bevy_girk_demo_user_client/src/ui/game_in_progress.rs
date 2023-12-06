//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy_fn_plugin::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_in_progress(ui: &mut UiBuilder<MainUi>)
{
    // prep overlay
    let window_overlay = make_overlay(ui.tree(), &Widget::new("root"), "", false);
    window_overlay.fetch_mut(ui.tree()).unwrap().get_container_mut().set_render_depth(Modifier::Set(750.));

    // style
    let style = ui.style::<GameInProgressStyle>();

    // add screen
    let barrier = relative_widget(ui.tree(), window_overlay.end(""), (-10., 110.), (-10., 110.));
    let barrier_img = ImageElementBundle::new(
            &barrier,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.background_color),
            ui.asset_server.load(style.background_img.0),
            style.background_img.1
        );
    ui.commands().spawn(barrier_img);
    ui.commands().spawn((barrier, UiInteractionBarrier::<MainUi>::default()));

    // add text
    ui.div_rel(window_overlay.end(""), (40., 60.), (45., 55.), move |ui, area| {
        // set text style
        ui.add_style(style.text.clone());

        // spawn text
        spawn_basic_text(ui, area.clone(), TextParams::center(), "Game In Progress");
    });

    // show overlay when a game is in progress
    ui.rcommands.on(resource_mutation::<GameMonitor>(),
            move |mut ui: UiUtils<MainUi>, game_monitor: ReactRes<GameMonitor>|
            {
                let enable = game_monitor.is_running();
                ui.toggle(enable, &window_overlay);
            }
        );

    // initialize ui
    ui.rcommands.trigger_resource_mutation::<GameMonitor>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiGameInProgressPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
