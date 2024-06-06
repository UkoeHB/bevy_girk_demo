use std::fmt::Write;

use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_demo_ui_prefab::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

fn add_fps(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // text layout helper
    let layout_helper = Widget::create(
        ui.tree(),
        area.end(""),
        RelativeLayout {
            //add slight buffer around edge; extend y-axis to avoid resizing issues
            absolute_1: Vec2 { x: 5., y: 6. },
            absolute_2: Vec2 { x: -5., y: 0. },
            relative_1: Vec2 { x: 0., y: 0. },
            relative_2: Vec2 { x: 100., y: 200. },
            ..Default::default()
        },
    )
    .unwrap();

    // make fps text
    let fps_text = Widget::create(
        ui.tree(),
        layout_helper.end(""),
        SolidLayout::new() //keep text in top right corner when window is resized
            .with_horizontal_anchor(1.0)
            .with_vertical_anchor(-1.0),
    )
    .unwrap();
    let text_entity = spawn_basic_text(ui, fps_text, TextParams::topleft(), "FPS: 999");

    // update text when fps changes
    ui.rcommands.on(
        resource_mutation::<FpsTracker>(),
        move |mut text: TextEditor, fps_tracker: ReactRes<FpsTracker>| {
            // only refresh once per second
            if fps_tracker.current_time().as_secs() <= fps_tracker.previous_time().as_secs() {
                return;
            }

            // refresh text
            text.write(text_entity, |text| write!(text, "FPS: {}", fps_tracker.fps()));
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_header(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), (80., 100.), (0., 100.), add_fps);
}

//-------------------------------------------------------------------------------------------------------------------

pub struct UiGameHeaderPlugin;

impl Plugin for UiGameHeaderPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(FpsTrackerPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
