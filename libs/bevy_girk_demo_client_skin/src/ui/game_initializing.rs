//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_client_fw::*;
use bevy_girk_game_fw::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Debug)]
struct InitializingOverlay
{
    overlay: Widget,
    loading_bar: Widget,
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn activate_initializing_overlay(mut ui: UiUtils<MainUi>, overlay: Res<InitializingOverlay>)
{
    ui.toggle(true, &overlay.overlay);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn deactivate_initializing_overlay(mut ui: UiUtils<MainUi>, overlay: Res<InitializingOverlay>)
{
    ui.toggle(false, &overlay.overlay);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn update_loading_bar(
    mut ui   : UiBuilder<MainUi>,
    overlay  : Res<InitializingOverlay>,
    progress : Query<&GameInitProgress>,
){
    let progress = progress
        .get_single()
        .map(|p| *p)
        .unwrap_or(GameInitProgress::default())
        .0
        .max(0.0)
        .min(100.0);

    // set bar width equal to the progress
    let Ok(bar_branch) = overlay.loading_bar.fetch_mut(ui.tree())
    else { tracing::error!("loading bar missing in ui tree"); return; };

    let LayoutPackage::Relative(layout) = bar_branch.get_container_mut().get_layout_mut()
    else { tracing::error!("loading bar not relative layout"); return; };

    layout.relative_2.x = progress * 100.;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_game_initializing(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // prep overlay
    let overlay = make_overlay(ui.tree(), area, "", false);
    overlay.fetch_mut(ui.tree()).unwrap().get_container_mut().set_render_depth(Modifier::Set(750.));

    // style
    let style = ui.style::<GameInitializingStyle>();

    // add screen
    let barrier = relative_widget(ui.tree(), overlay.end(""), (-10., 110.), (-10., 110.));
    let barrier_img = ImageElementBundle::new(
            &barrier,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.background_color),
            ui.asset_server.load(style.background_img.0),
            style.background_img.1
        );
    ui.commands().spawn(barrier_img);
    ui.commands().spawn((barrier, UiInteractionBarrier::<MainUi>::default()));

    // add text
    let text_style = style.text.clone();
    ui.div_rel(overlay.end(""), (40., 60.), (35., 45.), move |ui, area| {
        // set text style
        ui.add_style(text_style);

        // spawn text
        spawn_basic_text(ui, area.clone(), TextParams::center(), "Loading...");
    });

    // add loading bar box
    let bar_box = relative_widget(ui.tree(), overlay.end(""), (30., 70.), (50., 70.));
    let bar_box_img = ImageElementBundle::new(
            &bar_box,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.))
                //.with_depth(0.1)
                .with_color(style.loading_bar_box_color),
            ui.asset_server.load(style.loading_bar_box_img.0),
            style.loading_bar_box_img.1
        );
    ui.commands().spawn(bar_box_img);

    // add loading bar
    let loading_bar_frame = relative_widget(ui.tree(), bar_box.end(""), (0.5, 99.5), (0.5, 99.5));
    let loading_bar = relative_widget(ui.tree(), loading_bar_frame.end(""), (0., 0.), (0., 100.));
    let loading_bar_img = ImageElementBundle::new(
            &loading_bar,
            ImageParams::center()
                .with_width(Some(100.))
                .with_height(Some(100.))
                .with_color(style.loading_bar_color),
            ui.asset_server.load(style.loading_bar_img.0),
            style.loading_bar_img.1
        );
    ui.commands().spawn(loading_bar_img);

    // insert overlay resource
    ui.commands().insert_resource(InitializingOverlay{ overlay, loading_bar });
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiInitializingPlugin(app: &mut App)
{
    app.add_systems(OnEnter(ClientFWMode::Init), activate_initializing_overlay)
        .add_systems(OnExit(ClientFWMode::Init), deactivate_initializing_overlay)
        .add_systems(Update,
            update_loading_bar
                .in_set(ClientFWTickSet::End)
                //.in_set(ClientSet::InitCore)
        );
}

//-------------------------------------------------------------------------------------------------------------------
