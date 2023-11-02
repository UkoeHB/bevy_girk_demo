//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::borrow::Borrow;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn basic_relative_widget(
    ctx     : &mut UiBuilderCtx,
    path    : impl Borrow<str>,
    x_range : (f32, f32),
    y_range : (f32, f32)
) -> Widget
{
     Widget::create(
            ctx.ui,
            path,
            RelativeLayout{
                relative_1 : Vec2 { x: x_range.0, y: y_range.0 },
                relative_2 : Vec2 { x: x_range.1, y: y_range.1 },
                ..Default::default()
            }
        ).unwrap()
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn spawn_basic_text(ctx: UiBuilderCtx,
    widget                 : Widget,
    color                  : Color,
    (depth, width, height) : (Option<f32>, Option<f32>, Option<f32>),
    default_text           : &str,
) -> Entity
{
    let text_style = TextStyle {
            font      : ctx.asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : color,
        };

    let mut text_params = TextParams::center()
        .with_style(&text_style)
        .with_width(width)
        .with_height(height);
    if let Some(depth) = depth { text_params = text_params.with_depth(depth); }

    let text_entity = ctx.rcommands.commands().spawn(TextElementBundle::new(widget, text_params, default_text)).id();

    text_entity    
}

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
