use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_girk_demo_ui_prefab::*;
use bevy_kot::prelude::*;
//use bevy_girk_client_fw::*;
use bevy_lunex::prelude::*;

use crate::*;
//use bevy_lunex_ui::prelude::LunexUiSystemSet2D;

//-------------------------------------------------------------------------------------------------------------------

fn add_backdrop(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.add_style(ui.style::<GameClientBackdrop>().plain_box.clone());
    spawn_plain_box(ui, area.clone());
}

//-------------------------------------------------------------------------------------------------------------------

fn build_ui(mut ui: UiBuilder<MainUi>)
{
    // set base styles
    ui.add_style(ClientSkinStyles::default());

    // root widget
    let root = relative_widget(ui.tree(), "root", (0., 100.), (0., 100.));

    // backdrop
    ui.div_rel(root.end(""), (-5000., 5000.), (-5000., 5000.), add_backdrop);

    // contents
    ui.div(|ui| add_game_initializing(ui, &root));
    ui.div(|ui| add_game(ui, &root));
    ui.div(|ui| add_game_over(ui, &root));
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
            .add_plugins(LunexUiPlugin2D::<MainUi>::new())
            //.configure_sets(Update, LunexUiSystemSet2D.after(ClientFwTickSet::End))  //todo: need lunex update
            .register_interaction_source(MouseLButtonMain::default())
            .add_systems(PreStartup, setup_ui)
            .add_systems(Startup, build_ui)

            // ui plugins
            .add_plugins(UiLoadingPlugin)
            .add_plugins(UiGamePlugin)
            .add_plugins(UiGameOverPlugin)

            //.add_plugins(UIDebugOverlayPlugin)  //DEBUG ONLY
            ;
    }
}

//-------------------------------------------------------------------------------------------------------------------
