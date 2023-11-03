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

fn setup_ui_tree(
    mut rcommands : ReactCommands,
    asset_server  : Res<AssetServer>,
    mut main_ui   : Query<&mut UiTree, With<MainUI>>
){
    // prep builder context
    let ctx = &mut UiBuilderCtx{
            rcommands    : &mut rcommands,
            asset_server : &asset_server,
            ui           : &mut main_ui.get_single_mut().unwrap(),
        };

    // root widget
    let root = relative_widget(ctx, "root", (0., 100.), (0., 100.));

    // root zones
    // - play button (top left)
    let play_button = relative_widget(ctx, root.end("play_button"), (0., 20.), (0., 10.));
    ctx.commands().spawn((play_button.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - menu bar (center top)
    let menu_bar = relative_widget(ctx, root.end("menu_bar"), (20., 90.), (0., 10.));
    ctx.commands().spawn((menu_bar.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - menu item overlay area (everything below the menu bar)
    let menu_overlay = relative_widget(ctx, root.end("menu_overlay"), (0., 100.), (10., 100.));
    ctx.commands().spawn((menu_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    // - connection status (upper right corner)
    let status = relative_widget(ctx, root.end("status"), (90., 100.), (0., 10.));


    // add child widgets
    add_play_section(ctx, play_button, menu_overlay.clone());
    add_menu_bar_section(ctx, menu_bar, menu_overlay);
    add_status_section(ctx, status);
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
    let ui = UiTree::new("ui");
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
