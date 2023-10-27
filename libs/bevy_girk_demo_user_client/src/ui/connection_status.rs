//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::ecs::*;
use bevy_fn_plugin::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Component)]
struct ConnectionStatusFlag;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn refresh_status_text(
    mut status_text : Query<&mut Text, With<ConnectionStatusFlag>>,
    status          : Res<ConnectionStatus>,
){
    if !status.is_changed() { return; }
    let text_section = &mut status_text.single_mut().sections[0].value;
    text_section.clear();
    let _ = write!(text_section, "{}", status.to_str());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_status_section(rcommands: &mut ReactCommands, asset_server: &AssetServer, ui: &mut UiTree, text_base: Widget)
{
    // text layout helper
    let layout_helper = Widget::create(
            ui,
            text_base.end(""),
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
            ui,
            layout_helper.end(""),
            SolidLayout::new()  //keep text in top right corner when window is resized
                .with_horizontal_anchor(1.0)
                .with_vertical_anchor(-1.0),
        ).unwrap();

    let text_style = TextStyle {
            font      : asset_server.load(STATUS_FONT),
            font_size : 45.0,
            color     : STATUS_FONT_COLOR,
        };

    rcommands.commands().spawn(
            (
                TextElementBundle::new(
                    text,
                    TextParams::topright().with_style(&text_style),
                    "Connecting..."  //use initial value to get correct initial text boundary
                ),
                ConnectionStatusFlag
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiConnectionStatusPlugin(app: &mut App)
{
    app.add_systems(Update, refresh_status_text);
}

//-------------------------------------------------------------------------------------------------------------------
