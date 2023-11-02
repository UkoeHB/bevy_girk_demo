//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_basic_button(
    commands         : &mut Commands,
    asset_server     : &AssetServer,
    ui               : &mut UiTree,
    button_overlay   : &Widget,
    button_entity    : Entity,
    button_text      : &'static str,
    unpress_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
){
    // add default button image
    let default_button = make_overlay(ui, &button_overlay, "", true);
    commands.spawn(
            ImageElementBundle::new(
                    &default_button,
                    ImageParams::center()
                        .with_depth(50.)
                        .with_width(Some(100.))
                        .with_height(Some(100.))
                        .with_color(Color::GRAY),
                    asset_server.load(MISC_BUTTON),
                    Vec2::new(250.0, 142.0)
                )
        );

    // add pressed button image
    let pressed_button = make_overlay(ui, &button_overlay, "", false);
    commands.spawn(
            ImageElementBundle::new(
                    &pressed_button,
                    ImageParams::center()
                        .with_depth(50.)
                        .with_width(Some(100.))
                        .with_height(Some(100.))
                        .with_color(Color::DARK_GRAY),
                    asset_server.load(MISC_BUTTON),
                    Vec2::new(250.0, 142.0)
                )
        );

    // button entity
    let mut entity_commands = commands.get_entity(button_entity).unwrap();

    // add button text
    let text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 40.0,
            color     : MISC_FONT_COLOR,
        };

    entity_commands.insert(
            TextElementBundle::new(
                    button_overlay,
                    TextParams::center()
                        .with_style(&text_style)
                        .with_depth(100.)
                        .with_height(Some(40.)),
                    button_text
                )
        );

    // interactivity
    InteractiveElementBuilder::new()
        .with_default_widget(default_button)
        .with_pressed_widget(pressed_button)
        .press_on_click()
        .unpress_on_unclick_home_and_abort_on_unclick_away()
        .abort_press_if_obstructed()
        .unpress_callback(move |world, _| (unpress_callback)(world))
        .build::<MouseLButtonMain>(&mut entity_commands, button_overlay.clone())
        .unwrap();
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_basic_popup(
    commands        : &mut Commands,
    asset_server    : &AssetServer,
    ui              : &mut UiTree,
    window_scaling  : Vec2,  // x: % of screen width, y: % of screen height
    cancel_text     : &'static str,
    accept_text     : &'static str,
    cancel_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
    accept_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
) //-> (Widget, Widget)
{
    // popup overlay attached to root of ui tree
    let default_button = make_overlay(ui, &Widget::new("root"), "", true);

    // add screen-wide barrier

    // window box

    // region for caller's content

    // cancel button

    // accept button
}

//-------------------------------------------------------------------------------------------------------------------
