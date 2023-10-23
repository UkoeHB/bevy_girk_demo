//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_kot::ecs::*;
use bevy_kot::misc::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Component)]
struct FPSIndicator;

/// Realtime systems
fn refresh_fps_indicator(
    mut indicator_query : Query<&mut Text, With<FPSIndicator>>,
    fps_tracker         : Res<FPSTracker>
){
    // 1. only refresh once per second
    if fps_tracker.current_time().as_secs() <= fps_tracker.previous_time().as_secs()
        { return }

    // 2. refresh
    let indicator_value = &mut indicator_query.single_mut().sections[0].value;
    *indicator_value = format!("FPS: {}", fps_tracker.fps());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_fps_section(commands: &mut Commands, ui: &mut UiTree, fps: Widget, asset_server: &AssetServer)
{
    // fps layout helper
    let layout_helper = Widget::create(
            ui,
            fps.end(""),
            RelativeLayout{  //add slight buffer around edge; extend y-axis to avoid resizing issues
                absolute_1: Vec2 { x: 5., y: 5. },
                absolute_2: Vec2 { x: -5., y: 0. },
                relative_1: Vec2 { x: 0., y: 0. },
                relative_2: Vec2 { x: 100., y: 200. },
                ..Default::default()
            }
        ).unwrap();

    // fps text widget
    let fps_text = Widget::create(
            ui,
            layout_helper.end(""),
            SolidLayout::new()
                .with_horizontal_anchor(1.0)
                .with_vertical_anchor(-1.0),
        ).unwrap();

    let fps_text_style = TextStyle {
            font      : asset_server.load(FPS_FONT),
            font_size : 45.0,
            color     : FPS_FONT_COLOR,
        };

    commands.spawn(
            (
                TextElementBundle::new(
                    fps_text,
                    TextParams::topleft().with_style(&fps_text_style),
                    "FPS: 999"  //use initial value to get correct initial text boundary
                ),
                FPSIndicator
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_ui_tree(mut commands: Commands, mut main_ui: Query<&mut UiTree, With<MainUI>>, asset_server: Res<AssetServer>)
{
    let ui = &mut main_ui.get_single_mut().unwrap();

    // root widget
    let root = Widget::create(
            ui,
            "root",
            RelativeLayout{
                relative_1 : Vec2 { x: 0.0, y: 0.0 },
                relative_2 : Vec2 { x: 100.0, y: 100.0 },
                ..Default::default()
            }
        ).unwrap();


    // root zones
    // - play button
    let play_button = Widget::create(
            ui,
            root.end("play_button"),
            RelativeLayout{  //top left
                relative_1: Vec2 { x: 0., y: 0. },
                relative_2: Vec2 { x: 20., y: 10. },
                ..Default::default()
            }
        ).unwrap();
    commands.spawn((play_button.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - menu bar widget
    let menu_bar = Widget::create(
            ui,
            root.end("menu_bar"),
            RelativeLayout{  //center top
                relative_1: Vec2 { x: 20., y: 0. },
                relative_2: Vec2 { x: 90., y: 10. },
                ..Default::default()
            }
        ).unwrap();
    commands.spawn((menu_bar.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - menu item overlay area
    let menu_overlay = Widget::create(
            ui,
            root.end("menu_overlay"),
            RelativeLayout{  //everything below the menu bar
                relative_1: Vec2 { x: 0., y: 10.0 },
                relative_2: Vec2 { x: 100., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    commands.spawn((menu_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - fps display
    let fps = Widget::create(
            ui,
            root.end("fps"),
            RelativeLayout{  //upper right corner
                relative_1: Vec2 { x: 90., y: 0. },
                relative_2: Vec2 { x: 100., y: 10. },
                ..Default::default()
            }
        ).unwrap();


    // add child widgets
    add_play_section(&mut commands, ui, play_button, menu_overlay.clone(), &asset_server);
    add_menu_bar_section(&mut commands, ui, menu_bar, menu_overlay, &asset_server);
    add_fps_section(&mut commands, ui, fps, &asset_server);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_ui(mut commands: Commands)
{
    // prepare 2D camera
    commands.spawn(
            Camera2dBundle{
                transform: Transform{
                    translation: Vec3 { x: 0., y: 0., z: 1000. },
                    ..default()
                },
                ..default()
            }
        );

    // make lunex cursor
    commands.spawn(
            (
                bevy_lunex::prelude::Cursor::new(0.0),
                Transform::default(),
                MainMouseCursor,
            )
        );

    // create lunex ui tree
    let ui = UiTree::new("ui");

    // add ui tree to ecs
    commands.spawn((ui, MainUI));

    // initialize ui tree
    // - we do this after spawning the ui tree so that initialization can add commands that query the ui tree
    commands.add(|world: &mut World| syscall(world, (), setup_ui_tree) );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UIPlugin(app: &mut App)
{
    app
        .add_plugins(LunexUiPlugin)
        .add_plugins(FPSTrackerPlugin)
        .register_interaction_source(MouseLButtonMain::default())  //adds systems to First
        .add_systems(Startup,
            (
                setup_ui,
                apply_deferred,
            ).chain()
        )
        .add_systems(PreUpdate,
            (
                deselect_main_play_button_for_menu_button,
                deselect_main_menu_button_for_play_button,
                apply_deferred,
            ).chain()
        )
        .add_systems(Update, refresh_fps_indicator.after(FPSTrackerSet))

        .add_plugins(UIDebugOverlayPlugin)  //DEBUG ONLY
        ;
}

//-------------------------------------------------------------------------------------------------------------------
