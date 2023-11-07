//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_kot::prelude::*;
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_ui_tree(mut ui: UiBuilder<MainUI>)
{
    let ui = &mut ui;

    // root widget
    let root = relative_widget(ui.tree(), "root", (0., 100.), (0., 100.));

    // root zones
    // - play button (top left)
    let play_button = relative_widget(ui.tree(), root.end("play_button"), (0., 20.), (0., 10.));
    ui.commands().spawn((play_button.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - menu bar (center top)
    let menu_bar = relative_widget(ui.tree(), root.end("menu_bar"), (20., 90.), (0., 10.));
    ui.commands().spawn((menu_bar.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - add separators
    //todo: this is very janky
    let play_vertical = relative_widget(ui.tree(), root.end("play_vertical"), (-2., 20.), (-5., 10.));
    spawn_plain_outline(ui, play_vertical, None);
    let header_underline = relative_widget(ui.tree(), root.end("header_underline"), (-10., 110.), (-10., 10.));
    spawn_plain_outline(ui, header_underline, None);

    // - menu item overlay area (everything below the menu bar)
    let menu_overlay = relative_widget(ui.tree(), root.end("menu_overlay"), (0., 100.), (10., 100.));
    ui.commands().spawn((menu_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - connection status (upper right corner)
    let status = relative_widget(ui.tree(), root.end("status"), (90., 100.), (0., 10.));


    // add child widgets
    add_play_section(ui, play_button, menu_overlay.clone());
    add_menu_bar_section(ui, menu_bar, menu_overlay);
    add_status_section(ui, status);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_ui(mut commands: Commands)
{
    // prepare 2D camera
    commands.spawn(
            Camera2dBundle{ transform: Transform{ translation: Vec3 { x: 0., y: 0., z: 1000. }, ..default() }, ..default() }
        );

    // make lunex cursor
    commands.spawn((bevy_lunex::prelude::Cursor::new(0.0), Transform::default(), MainMouseCursor));

    // add new ui tree to ecs
    commands.insert_resource(StyleStackRes::<MainUI>::default());
    commands.spawn((UiTree::new("ui"), MainUI));

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
