//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;
use bevy_fn_plugin::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_client_id_display(In(area): In<Widget>, mut ui: UiBuilder<MainUi>, client: Res<HostUserClient>)
{
    ui.div(|ui| {
        // set text style
        ui.add_style(basic_text_default_light_style());

        // text
        let text = Widget::create(
                ui.tree(),
                area.end(""),
                SolidLayout::new()  //keep text in top right corner when window is resized
                    .with_horizontal_anchor(1.0)
                    .with_vertical_anchor(-1.0),
            ).unwrap();

        spawn_basic_text(ui, text, TextParams::topleft(), format!("ID: {}", client.id() % 1_000_000u128).as_str());
    });
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_info_section(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // text layout helper
    let layout_helper = Widget::create(
            ui.tree(),
            area.end(""),
            RelativeLayout{  //add slight buffer around edge; extend y-axis to avoid resizing issues
                absolute_1: Vec2 { x: 5., y: 6. },
                absolute_2: Vec2 { x: -5., y: 0. },
                relative_1: Vec2 { x: 0., y: 0. },
                relative_2: Vec2 { x: 100., y: 200. },
                ..Default::default()
            }
        ).unwrap();

    // text widget
    let layout_helper_clone = layout_helper.clone();
    ui.commands().add(move |world: &mut World| syscall(world, layout_helper, setup_client_id_display));

    // attach status to bottom of client id
    let offshoot = relative_widget(ui.tree(), layout_helper_clone.end(""), (0., 100.), (19., 119.));
    let status_text = Widget::create(
            ui.tree(),
            offshoot.end(""),
            SolidLayout::new()  //keep text in top right corner when window is resized
                .with_horizontal_anchor(1.0)
                .with_vertical_anchor(-1.0),
        ).unwrap();
    let text_entity = spawn_basic_text(ui, status_text, TextParams::topleft(), "Connecting...");

    // update text when connection status changes
    ui.rcommands.on(resource_mutation::<ConnectionStatus>(),
            move |mut text: TextHandle, status: ReactRes<ConnectionStatus>|
            {
                text.write(text_entity, 0, |text| write!(text, "{}", status.to_str())).unwrap();
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiConnectionStatusPlugin(_app: &mut App)
{}

//-------------------------------------------------------------------------------------------------------------------
