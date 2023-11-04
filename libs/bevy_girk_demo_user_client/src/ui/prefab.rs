//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::ecs::*;
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
            ctx.ui(),
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

pub(crate) fn spawn_plain_box(ctx: &mut UiBuilderCtx, widget: Widget, depth: Option<f32>) -> Entity
{
    let image = ImageElementBundle::new(
            &widget,
            ImageParams::center()
                .with_depth(depth.unwrap_or_default())  //todo: remove when lunex depth bug is fixed
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ctx.asset_server.load(BOX),
            Vec2::new(236.0, 139.0)
        );
    let box_entity = ctx.commands().spawn(image).id();

    box_entity
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn spawn_plain_outline(ctx: &mut UiBuilderCtx, widget: Widget, depth: Option<f32>) -> Entity
{
    let image = ImageElementBundle::new(
            &widget,
            ImageParams::center()
                .with_depth(depth.unwrap_or_default())  //todo: remove when lunex depth bug is fixed
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
    let default_button = make_overlay(ctx.ui(), &button_overlay, "", true);
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
    let pressed_button = make_overlay(ctx.ui(), &button_overlay, "", false);
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

#[derive(Debug, Clone)]
pub(crate) struct BasicPopupPack
{
    pub(crate) window_overlay  : Widget,
    pub(crate) content_section : Widget,
    pub(crate) cancel_button   : Widget,
    pub(crate) cancel_entity   : Entity,
    pub(crate) accept_button   : Widget,
    pub(crate) accept_entity   : Entity,
}

/// Make a basic popup.
///
/// The popup is a square with cancel and accept buttons. The button text and behavior can be customized. Pressing the
/// cancel button will set the popup overlay visibility to `false`. You should set it to `true` manually in order to
/// 'activate' the popup.
///
/// The popup is attached to the `"root"` widget. This function will panic if the root of your main ui tree does not
/// contain a widget with that name.
///
/// The popup is arbitrarily set at a depth of `500.0` in order to supercede the normal UI tree. Multiple concurrent
/// popups will NOT stack on each other properly.
pub(crate) fn spawn_basic_popup(
    ctx             : &mut UiBuilderCtx,
    window_scaling  : (f32, f32),  // (% of screen width, % of screen height)
    cancel_text     : &'static str,
    accept_text     : &'static str,
    cancel_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
    accept_callback : impl Fn(&mut World) -> () + Send + Sync + 'static,
) -> BasicPopupPack
{
    // popup overlay attached to root of ui tree
    let window_overlay = make_overlay(ctx.ui(), &Widget::new("root"), "", false);

    // add screen-wide barrier
    let barrier = relative_widget(ctx, window_overlay.end(""), (-10., 110.), (-10., 110.));
    let barrier_img = ImageElementBundle::new(
            &barrier,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ctx.asset_server.load(FILM),
            Vec2::new(236.0, 139.0)
        );
    ctx.commands().spawn(barrier_img);
    ctx.commands().spawn((barrier, UIInteractionBarrier::<MainUI>::default()));

    // window box
    let xmod = window_scaling.0.max(0.).min(100.) / 2.;
    let ymod = window_scaling.1.max(0.).min(100.) / 2.;
    let window = relative_widget(ctx, window_overlay.end(""), (50. - xmod, 50. + xmod), (50. - ymod, 50. + ymod));
    let window_img = ImageElementBundle::new(
            &window,
            ImageParams::center()
                .with_depth(0.1)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(Color::AZURE),
            ctx.asset_server.load(BOX),
            Vec2::new(236.0, 139.0)
        );
    ctx.commands().spawn((window_img, UIInteractionBarrier::<MainUI>::default()));

    // region for caller's content
    let content_section = relative_widget(ctx, window.end(""), (0., 100.), (0., 82.));

    // cancel button
    let cancel_button = relative_widget(ctx, window.end(""), (15., 27.), (86., 94.));
    let cancel_entity = ctx.commands().spawn_empty().id();
    let window_overlay_clone = window_overlay.clone();
    make_basic_button(ctx, &cancel_button, cancel_entity, cancel_text,
            move |world|
            {
                syscall(world, (MainUI, [], [window_overlay_clone.clone()]), toggle_ui_visibility);
                (cancel_callback)(world);
            }
        );

    // accept button
    let accept_button = relative_widget(ctx, window.end(""), (73., 85.), (86., 94.));
    let accept_entity = ctx.commands().spawn_empty().id();
    make_basic_button(ctx, &accept_button, accept_entity, accept_text, move |world| (accept_callback)(world));

    // set depth of window above all other ui elements
    let branch = window_overlay.fetch_mut(ctx.ui()).unwrap();
    branch.set_depth(500.);

    BasicPopupPack{
            window_overlay,
            content_section,
            cancel_button,
            cancel_entity,
            accept_button,
            accept_entity,
        }
}

//-------------------------------------------------------------------------------------------------------------------
