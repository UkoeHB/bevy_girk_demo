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

pub(crate) fn relative_widget(
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

/// The text style of `params` will be overwritten.
pub(crate) fn spawn_basic_text(
    ctx          : &mut UiBuilderCtx,
    widget       : Widget,
    color        : Color,
    params       : TextParams,
    default_text : &str,
) -> Entity
{
    let text_style = TextStyle {
            font      : ctx.asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : color,
        };

    let text_params = params.with_style(&text_style);
    let text_entity = ctx.commands().spawn(TextElementBundle::new(widget, text_params, default_text)).id();

    text_entity    
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn spawn_plain_box(ctx: &mut UiBuilderCtx, widget: Widget) -> Entity
{
    let image = ImageElementBundle::new(
            &widget,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ctx.asset_server.load(BOX),
            Vec2::new(236.0, 139.0)
        );
    let box_entity = ctx.commands().spawn(image).id();

    box_entity
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn spawn_plain_outline(ctx: &mut UiBuilderCtx, widget: Widget) -> Entity
{
    let image = ImageElementBundle::new(
            &widget,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ctx.asset_server.load(OUTLINE),
            Vec2::new(236.0, 139.0)
        );
    let box_entity = ctx.commands().spawn(image).id();

    box_entity
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_basic_button(
    ctx              : &mut UiBuilderCtx,
    button_overlay   : &Widget,
    button_entity    : Entity,
    button_text      : &'static str,
    unpress_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
){
    // add default button image
    let default_button = make_overlay(ctx.ui, &button_overlay, "", true);
    let default_image = ImageElementBundle::new(
            &default_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(Color::GRAY),
            ctx.asset_server.load(MISC_BUTTON),
            Vec2::new(250.0, 142.0)
        );
    ctx.commands().spawn(default_image);

    // add pressed button image
    let pressed_button = make_overlay(ctx.ui, &button_overlay, "", false);
    let pressed_image = ImageElementBundle::new(
            &pressed_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(Color::DARK_GRAY),
            ctx.asset_server.load(MISC_BUTTON),
            Vec2::new(250.0, 142.0)
        );
    ctx.commands().spawn(pressed_image);

    // add button text
    let text_style = TextStyle {
            font      : ctx.asset_server.load(MISC_FONT),
            font_size : 40.0,
            color     : MISC_FONT_COLOR,
        };

    let mut entity_commands = ctx.commands().get_entity(button_entity).unwrap();
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

    // build button
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
    ctx             : &mut UiBuilderCtx,
    window_scaling  : Vec2,  // x: % of screen width, y: % of screen height
    cancel_text     : &'static str,
    accept_text     : &'static str,
    cancel_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
    accept_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
) //-> (Widget, Widget)
{
    // popup overlay attached to root of ui tree
    let default_button = make_overlay(ctx.ui, &Widget::new("root"), "", true);

    // add screen-wide barrier

    // window box

    // region for caller's content

    // cancel button

    // accept button
}

//-------------------------------------------------------------------------------------------------------------------
