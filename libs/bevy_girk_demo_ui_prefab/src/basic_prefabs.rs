//local shortcuts
use crate::*;
use bevy_kot_ui::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot_ecs::*;
use bevy_kot_ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Spawn a text element in the specified widget.
///
/// Uses style [`BasicText`].
///
/// The `TextStyle` of `params` will be overwritten.
pub fn spawn_basic_text(
    ui           : &mut UiBuilder<MainUi>,
    widget       : Widget,
    params       : TextParams,
    initial_text : &str,
) -> Entity
{
    let style = ui.style::<BasicText>();

    //todo: add widget
    let text_style = TextStyle {
            font      : ui.asset_server.load(style.font),
            font_size : 45.0,
            color     : style.color,
        };

    let text_params = params.with_style(&text_style);
    let text_entity = ui.commands().spawn(TextElementBundle::new(widget, text_params, initial_text)).id();

    text_entity
}

//-------------------------------------------------------------------------------------------------------------------

/// Spawn a box in the specified widget.
///
/// Uses style [`PlainBox`].
pub fn spawn_plain_box(ui: &mut UiBuilder<MainUi>, widget: Widget) -> Entity
{
    let style = ui.style::<PlainBox>();

    //todo: add widget
    let image = ImageElementBundle::new(
            &widget,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ui.asset_server.load(style.img.0),
            style.img.1
        );
    let box_entity = ui.commands().spawn(image).id();

    box_entity
}

//-------------------------------------------------------------------------------------------------------------------

/// Spawn an outline in the specified widget.
///
/// Uses style [`PlainOutline`].
pub fn spawn_plain_outline(ui: &mut UiBuilder<MainUi>, widget: Widget) -> Entity
{
    let style = ui.style::<PlainOutline>();

    //todo: add widget
    let image = ImageElementBundle::new(
            &widget,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ui.asset_server.load(style.img.0),
            style.img.1
        );
    let outline_entity = ui.commands().spawn(image).id();

    outline_entity
}

//-------------------------------------------------------------------------------------------------------------------

/// Spawn a basic button in the specified widget.
///
/// Uses style [`BasicButton`]
pub fn spawn_basic_button<Marker>(
    ui               : &mut UiBuilder<MainUi>,
    button_overlay   : &Widget,
    button_text      : &'static str,
    unpress_callback : impl IntoSystem<(), (), Marker> + Send + Sync + 'static,
) -> Entity
{
    let style = ui.style::<BasicButton>();

    // add default button image
    let default_button = make_overlay(ui.tree(), &button_overlay, "", true);
    let default_image = ImageElementBundle::new(
            &default_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.default_img_color),
            ui.asset_server.load(style.default_img.0),
            style.default_img.1
        );
    ui.commands().spawn(default_image);

    // add pressed button image
    let pressed_button = make_overlay(ui.tree(), &button_overlay, "", false);
    let pressed_image = ImageElementBundle::new(
            &pressed_button,
            ImageParams::center()
                .with_depth(50.)
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.pressed_img_color),
            ui.asset_server.load(style.pressed_img.0),
            style.pressed_img.1
        );
    ui.commands().spawn(pressed_image);

    // add button text
    let text_style = TextStyle {
            font      : ui.asset_server.load(style.text.font),
            font_size : 40.0,
            color     : style.text.color,
        };

    let despawner = ui.despawner.clone();
    let mut entity_commands = ui.commands().spawn_empty();
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
        .on_unpress(unpress_callback)
        .build::<MouseLButtonMain>(&despawner, &mut entity_commands, button_overlay.clone())
        .unwrap();

    entity_commands.id()
}

//-------------------------------------------------------------------------------------------------------------------

pub fn spawn_basic_button_blocker(ui: &mut UiBuilder<MainUi>, area: &Widget, visible: bool) -> Widget
{
    let disable_overlay = make_overlay(ui.tree(), &area, "", visible);
    ui.commands().spawn((disable_overlay.clone(), UiInteractionBarrier::<MainUi>::default()));

    disable_overlay
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BasicPopupPack
{
    pub window_overlay  : Widget,
    pub content_section : Widget,
    pub cancel_button   : Widget,
    pub cancel_entity   : Entity,
    pub accept_button   : Widget,
    pub accept_entity   : Entity,
}

/// Make a basic popup.
///
/// The popup is a square with cancel and accept buttons. The button text and behavior can be customized. Pressing the
/// cancel button will set the popup overlay visibility to `false`. You should set it to `true` manually in order to
/// activate the popup. Note that the popup starts with visiblity `false`.
///
/// The popup is attached to the `"root"` widget. This function will panic if the root of your main ui tree does not
/// contain a widget with that name.
///
/// The popup is arbitrarily set at a depth of `500.0` in order to supercede the normal UI tree. Multiple concurrent
/// popups will NOT stack on each other properly.
pub fn spawn_basic_popup<Marker1, Marker2>(
    ui              : &mut UiBuilder<MainUi>,
    cancel_text     : &'static str,
    accept_text     : &'static str,
    cancel_callback : impl IntoSystem<(), (), Marker1> + Send + Sync + 'static,
    accept_callback : impl IntoSystem<(), (), Marker2> + Send + Sync + 'static,
) -> BasicPopupPack
{
    let style = ui.style::<BasicPopup>();

    // popup overlay attached to root of ui tree
    let window_overlay = make_overlay(ui.tree(), &Widget::new("root"), "", false);
    window_overlay.fetch_mut(ui.tree()).unwrap().get_container_mut().set_render_depth(Modifier::Set(500.));

    // add screen-wide barrier
    let barrier = relative_widget(ui.tree(), window_overlay.end(""), (-10., 110.), (-10., 110.));
    let barrier_img = ImageElementBundle::new(
            &barrier,
            ImageParams::center()
                .with_depth(-0.1)
                .with_width(Some(100.))
                .with_height(Some(100.)),
            ui.asset_server.load(style.background.0),
            style.background.1
        );
    ui.commands().spawn(barrier_img);
    ui.commands().spawn((barrier, UiInteractionBarrier::<MainUi>::default()));

    // window backdrop
    let xmod = style.proportions.x.max(0.).min(100.) / 2.;
    let ymod = style.proportions.y.max(0.).min(100.) / 2.;
    let window = relative_widget(ui.tree(), window_overlay.end(""), (50. - xmod, 50. + xmod), (50. - ymod, 50. + ymod));
    let window_img = ImageElementBundle::new(
            &window,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.backdrop_color),
            ui.asset_server.load(style.backdrop.0),
            style.backdrop.1
        );
    ui.commands().spawn((window_img, UiInteractionBarrier::<MainUi>::default()));

    // window border
    ui.div(|ui|{
        ui.add_style(style.border.clone());
        let window_border = make_overlay(ui.tree(), &window, "", true);
        spawn_plain_outline(ui, window_border);
    });

    // region for caller's content
    let content_percent = style.content_percent.max(0.).min(100.);
    let content_section = relative_widget(ui.tree(), window.end(""), (0., 100.), (0., content_percent));

    // region for buttons
    let buttons_section = relative_widget(ui.tree(), window.end(""), (0., 100.), (content_percent, 100.));
    let window_overlay_clone = window_overlay.clone();

    let button_spacing = style.button_spacing;
    let button_gap = style.button_gap;
    let button_width = (100. - (button_spacing * 3. + button_gap)) / 2.;
    let button_height = style.button_ratio * button_width
        * ((100. - style.button_dead_space) / 100.)
        / ((100. - content_percent) / 100.);
    let button_vertical_spacing = (100. - (button_height + style.button_dead_space)) / 2.;
    let (cancel_button, cancel_entity, accept_button, accept_entity) =
    ui.div(move |ui| {
        // add button style
        ui.add_style(style.button.clone());

        // cancel button
        let cancel_button = relative_widget(
                ui.tree(),
                buttons_section.end(""),
                (button_spacing, button_spacing + button_width),
                (button_vertical_spacing, button_vertical_spacing + button_height)
            );
        let mut cancel_callback = CallbackSystem::new(cancel_callback);
        let cancel_entity = spawn_basic_button(ui, &cancel_button, cancel_text,
                move |world: &mut World|
                {
                    syscall(world, (MainUi, [], [window_overlay_clone.clone()]), toggle_ui_visibility);
                    cancel_callback.run(world, ());
                }
            );

        // accept button
        let accept_button = relative_widget(
                ui.tree(),
                buttons_section.end(""),
                (100. - (button_spacing + button_width), 100. - button_spacing),
                (button_vertical_spacing, button_vertical_spacing + button_height)
            );
        let accept_entity = spawn_basic_button(ui, &accept_button, accept_text, accept_callback);

        (cancel_button, cancel_entity, accept_button, accept_entity)
    });

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
