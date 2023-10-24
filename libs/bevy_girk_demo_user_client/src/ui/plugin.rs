//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_kot::ecs::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_lunex::prelude::*;

//standard shortcuts


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

    // - menu bar
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

    // - connection status
    let status = Widget::create(
            ui,
            root.end("status"),
            RelativeLayout{  //upper right corner
                relative_1: Vec2 { x: 90., y: 0. },
                relative_2: Vec2 { x: 100., y: 10. },
                ..Default::default()
            }
        ).unwrap();


    // add child widgets
    add_play_section(&mut commands, &asset_server, ui, play_button, menu_overlay.clone());
    add_menu_bar_section(&mut commands, &asset_server, ui, menu_bar, menu_overlay);
    add_status_section(&mut commands, &asset_server, ui, status);
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
    commands.add(|world: &mut World| syscall(world, (), setup_ui_tree));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UIPlugin(app: &mut App)
{
    app
        .add_plugins(LunexUiPlugin)
        .register_interaction_source(MouseLButtonMain::default())
        .add_systems(Startup,
            (
                setup_ui, apply_deferred,
            ).chain()
        )

        // ui plugins
        .add_plugins(UiConnectionStatusPlugin)
        .add_plugins(UiPlaySectionPlugin)

        //.add_plugins(UIDebugOverlayPlugin)  //DEBUG ONLY
        ;
}

//-------------------------------------------------------------------------------------------------------------------
