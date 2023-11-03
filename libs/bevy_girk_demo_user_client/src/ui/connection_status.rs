//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::ecs::*;
use bevy_fn_plugin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_status_section(ctx: &mut UiBuilderCtx, area: Widget)
{
    // text layout helper
    let layout_helper = Widget::create(
            ctx.ui(),
            area.end(""),
            RelativeLayout{  //add slight buffer around edge; extend y-axis to avoid resizing issues
                absolute_1: Vec2 { x: 5., y: 6. },
                absolute_2: Vec2 { x: -9., y: 0. },
                relative_1: Vec2 { x: 0., y: 0. },
                relative_2: Vec2 { x: 100., y: 200. },
                ..Default::default()
            }
        ).unwrap();

    // text widget
    let text = Widget::create(
            ctx.ui(),
            layout_helper.end(""),
            SolidLayout::new()  //keep text in top right corner when window is resized
                .with_horizontal_anchor(1.0)
                .with_vertical_anchor(-1.0),
        ).unwrap();

    let text_entity = spawn_basic_text(
            ctx,
            text,
            STATUS_FONT_COLOR,
            TextParams::topright(),
            "Connecting..."
        );

    // update text when connection status changes
    ctx.rcommands.add_resource_mutation_reactor::<ConnectionStatus>(
            move |world: &mut World|
            {
                // define updated text
                let status_str = world.resource::<ReactRes<ConnectionStatus>>().to_str();
                let text = format!("{}", status_str);

                // update UI text
                syscall(world, (text_entity, text), update_ui_text);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiConnectionStatusPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
