use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn add_backdrop(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.add_style(ui.style::<UserClientBackdropStyle>().backdrop_box.clone());
    spawn_plain_box(ui, area.clone());
}

//-------------------------------------------------------------------------------------------------------------------

fn build_ui(mut ui: UiBuilder<MainUi>)
{
    // set base styles
    ui.add_style(UserClientStyles::new());

    // root widget
    let root = relative_widget(ui.tree(), "root", (0., 100.), (0., 100.));

    // backdrop
    ui.div_rel(root.end(""), (-5000., 5000.), (-5000., 5000.), add_backdrop);

    // root zones
    // - play button (top left)
    let play_button = relative_widget(ui.tree(), root.end("play_button"), (0., 20.), (0., 10.));
    ui.commands()
        .spawn((play_button.clone(), UiInteractionBarrier::<MainUi>::default()));

    // - menu bar (center top)
    let menu_bar = relative_widget(ui.tree(), root.end("menu_bar"), (20., 90.), (0., 10.));
    ui.commands()
        .spawn((menu_bar.clone(), UiInteractionBarrier::<MainUi>::default()));

    // - add separators
    //todo: this is very janky
    let play_vertical = relative_widget(ui.tree(), root.end("play_vertical"), (-2., 20.), (-5., 10.));
    let header_underline = relative_widget(ui.tree(), root.end("header_underline"), (-10., 110.), (-10., 10.));
    spawn_plain_outline(&mut ui, play_vertical);
    spawn_plain_outline(&mut ui, header_underline);

    // - menu item overlay area (everything below the menu bar)
    let menu_overlay = relative_widget(ui.tree(), root.end("menu_overlay"), (0., 100.), (10., 100.));
    ui.commands()
        .spawn((menu_overlay.clone(), UiInteractionBarrier::<MainUi>::default()));

    // - user info (upper right corner)
    let info = relative_widget(ui.tree(), root.end("info"), (90., 100.), (0., 10.));

    // add content sections
    ui.div(|ui| add_play_section(ui, &play_button, &menu_overlay));
    ui.div(|ui| add_menu_bar_section(ui, &menu_bar, &menu_overlay));
    ui.div(|ui| add_info_section(ui, &info));
    ui.div(|ui| add_ack_lobby_window(ui));
    ui.div(|ui| add_game_in_progress(ui));
}

//-------------------------------------------------------------------------------------------------------------------

fn setup_ui(mut commands: Commands, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>)
{
    // prepare 2D camera
    commands.spawn(Camera2dBundle {
        transform: Transform { translation: Vec3 { x: 0., y: 0., z: 1000. }, ..default() },
        ..default()
    });

    // make lunex cursor
    commands.spawn((
        Cursor::new(),
        Transform::default(),
        Visibility::default(),
        MainMouseCursor,
    ));

    // add new ui tree to ecs
    commands.insert_resource(StyleStackRes::<MainUi>::default());
    let tree = UiTree::<MainUi>::new("ui");

    let window = window.single();
    commands.entity(window).insert(tree.bundle());
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(PreStartup, setup_ui)
            .add_systems(Startup, build_ui)

            // ui plugins
            .add_plugins(MenuContentPlugin)
            .add_plugins(UiConnectionStatusPlugin)
            .add_plugins(UiPlaySectionPlugin)
            .add_plugins(UiAckLobbyWindowPlugin)
            .add_plugins(UiGameInProgressPlugin)

            //.add_plugins(UIDebugOverlayPlugin)  //DEBUG ONLY
            ;
    }
}

//-------------------------------------------------------------------------------------------------------------------
