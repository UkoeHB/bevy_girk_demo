use std::fmt::Write;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_backend_public::*;
use wiring_backend::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn setup_refresh_indicator_reactors(
    In(text_widget): In<Widget>,
    mut c: Commands,
    lobby_search: Query<Entity, With<LobbySearch>>,
)
{
    let lobby_search_entity = lobby_search.single();

    // activate text when there is a pending request
    let text_widget_clone = text_widget.clone();
    c.react().on(
        entity_insertion::<PendingRequest>(lobby_search_entity),
        move |mut ui: UiUtils<MainUi>| ui.toggle(true, &text_widget_clone),
    );

    // deactivate text when there is not a pending request
    c.react().on(
        entity_removal::<PendingRequest>(lobby_search_entity),
        move |mut ui: UiUtils<MainUi>| ui.toggle(false, &text_widget),
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn update_lobby_list_contents(
    In(contents): In<Arc<Vec<(Entity, Widget)>>>,
    mut ui: UiUtils<MainUi>,
    lobby_page: ReactRes<LobbyPage>,
)
{
    // lobby list entries
    let entries = lobby_page.get();

    // update contents
    for (idx, (content_entity, content_widget)) in contents.iter().enumerate() {
        // get lobby list entry
        let entry = entries.get(idx);

        // update entry visibility
        ui.toggle(entry.is_some(), &content_widget);

        // update entry text
        let Some(entry) = entry else {
            continue;
        };
        ui.text.write(*content_entity, |text| {
            write!(
                            text,
                            "Lobby: {:0>6}, Owner: {:0>6}, Players: {}/{}, Watchers: {}/{}",
                            entry.id % 1_000_000u64,
                            entry.owner_id % 1_000_000u128,
                            entry.num(ClickLobbyMemberType::Player),
                            entry.max(ClickLobbyMemberType::Player),
                            entry.num(ClickLobbyMemberType::Watcher),
                            entry.max(ClickLobbyMemberType::Watcher),
                        )
        });
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn open_join_lobby_window(In(lobby_list_index): In<usize>, mut c: Commands)
{
    // activate the window
    c.react()
        .broadcast(ActivateJoinLobbyWindow { lobby_list_index });
}

//-------------------------------------------------------------------------------------------------------------------

fn open_make_lobby_window(mut c: Commands)
{
    // activate the window
    c.react().broadcast(ActivateMakeLobbyWindow);
}

//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_title(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // title text
    let text = relative_widget(ui.tree(), area.end(""), (20., 80.), (40., 70.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(100.)), "Lobby List");
}

//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_indicator(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // make overlay so indicator visibility starts `false`
    let overlay = make_overlay(ui.tree(), &area, "", false);

    // set text style
    ui.add_style(basic_text_default_light_style());

    // indicator text
    let text = relative_widget(ui.tree(), overlay.end(""), (5., 98.), (15., 95.));
    spawn_basic_text(ui, text, TextParams::centerright().with_height(Some(40.)), "Loading...");

    // setup reactors
    ui.commands()
        .add(move |world: &mut World| world.syscall(overlay, setup_refresh_indicator_reactors));
}

//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    let button_overlay = relative_widget(ui.tree(), area.end(""), (5., 98.), (10., 90.));
    spawn_basic_button(ui, &button_overlay, "Refresh", refresh_lobby_list);
}

//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_subsection(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // backdrop
    ui.div(|ui| {
        ui.add_style(ui.style::<LobbyListStyle>().backdrop_box.clone());
        spawn_plain_box(ui, area.clone());
    });

    // list outline above backdrop
    let outline = make_overlay(ui.tree(), &area, "", true);
    spawn_plain_outline(ui, outline.clone());

    // prepare contents
    let mut contents = Vec::with_capacity(LOBBY_LIST_SIZE as usize);

    let entry_height = (1. / LOBBY_LIST_SIZE as f32) * 95.;

    for i in 0..LOBBY_LIST_SIZE {
        let content_widget = make_overlay(ui.tree(), area, "", true);

        // text
        let y_start = 2.5 + (i as f32) * entry_height;
        let y_end = 2.5 + ((i + 1) as f32) * entry_height;

        let text = relative_widget(ui.tree(), content_widget.end(""), (5., 78.), (y_start, y_end));

        let content_entity = spawn_basic_text(
            ui,
            text,
            TextParams::centerleft().with_width(Some(100.)),
            "Lobby: ??????, Owner: ??????, Players: 00/00, Watchers: 00/00",
        );

        // button
        let join_button_area = relative_widget(
            ui.tree(),
            content_widget.end(""),
            (80., 98.),
            (y_start + 0.5, y_end - 0.5),
        );
        spawn_basic_button(ui, &join_button_area, "Join", prep_fncall(i, open_join_lobby_window));

        // save this entry
        contents.push((content_entity, content_widget));
    }

    let contents = Arc::new(contents);

    // update contents when lobby page changes
    ui.commands().react().on(
        resource_mutation::<LobbyPage>(),
        prep_fncall(contents, update_lobby_list_contents),
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_stats(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // stats text
    let text_entity = spawn_basic_text(
        ui,
        area.clone(),
        TextParams::center().with_width(Some(100.)),
        "(????-???? / ????)",
    );

    // update stats when lobby page updates
    ui.commands().react().on(
        resource_mutation::<LobbyPage>(),
        move |mut ui: UiUtils<MainUi>, page: ReactRes<LobbyPage>| {
            let (first, last, total) = page.stats();
            ui.text
                .write(text_entity, |text| write!(text, "({}-{} / {})", first, last, total));
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_now_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    // - request the most recent possible lobby
    let button_entity = spawn_basic_button(ui, &area, "Now", request_lobby_list_now);

    // disable button when displaying 'now'
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.commands().react().on(
        resource_mutation::<LobbyPageRequest>(),
        move |mut ui: UiUtils<MainUi>, page_req: ReactRes<LobbyPageRequest>| {
            let enable = !page_req.is_now();
            ui.toggle_basic_button(enable, button_entity, &disable_overlay);
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_left_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    // - request the next most recent lobby
    let button_entity = spawn_basic_button(ui, &area, "<", request_lobby_list_next_newer);

    // disable button when no newer lobbies to request
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.commands().react().on(
        resource_mutation::<LobbyPage>(),
        move |mut ui: UiUtils<MainUi>, page: ReactRes<LobbyPage>| {
            let (first, _, _) = page.stats();
            let enable = first != 1;
            ui.toggle_basic_button(enable, button_entity, &disable_overlay);
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_right_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    // - request the next older lobby
    let button_entity = spawn_basic_button(ui, &area, ">", request_lobby_list_next_older);

    // disable button when no older lobbies to request
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.commands().react().on(
        resource_mutation::<LobbyPage>(),
        move |mut ui: UiUtils<MainUi>, page: ReactRes<LobbyPage>| {
            let (_, last, total) = page.stats();
            let enable = last != total;
            ui.toggle_basic_button(enable, button_entity, &disable_overlay);
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_oldest_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    // - request the oldest lobbies
    let button_entity = spawn_basic_button(ui, &area, "Oldest", request_lobby_list_oldest);

    // disable button when last requested the oldest lobbies
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.commands().react().on(
        resource_mutation::<LobbyPageRequest>(),
        move |mut ui: UiUtils<MainUi>, page_req: ReactRes<LobbyPageRequest>| {
            let enable = !page_req.is_oldest();
            ui.toggle_basic_button(enable, button_entity, &disable_overlay);
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

fn add_navigation_subsection(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // navigation info
    ui.div_rel(area.end(""), (38., 62.), (20., 60.), add_lobby_list_stats);

    // navigation buttons
    ui.div_rel(area.end(""), (1., 13.), (5., 95.), add_clamp_now_button);
    ui.div_rel(area.end(""), (14., 26.), (5., 95.), add_paginate_left_button);
    ui.div_rel(area.end(""), (74., 86.), (5., 95.), add_paginate_right_button);
    ui.div_rel(area.end(""), (87., 99.), (5., 95.), add_clamp_oldest_button);
}

//-------------------------------------------------------------------------------------------------------------------

fn add_new_lobby_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // area
    let button_area = relative_widget(ui.tree(), area.end(""), (25., 75.), (10., 90.));

    // button ui
    let button_overlay = make_overlay(ui.tree(), &button_area, "", true);
    let button_entity = spawn_basic_button(ui, &button_overlay, "New Lobby", open_make_lobby_window);

    // disable button when we are in a lobby already
    let disable_overlay = make_overlay(ui.tree(), &button_overlay, "", false);
    ui.commands()
        .spawn((disable_overlay.clone(), UiInteractionBarrier::<MainUi>::default()));

    ui.commands().react().on(
        resource_mutation::<LobbyDisplay>(),
        move |mut ui: UiUtils<MainUi>, display: ReactRes<LobbyDisplay>| {
            let enable = !display.is_set();
            ui.toggle_basic_button(enable, button_entity, &disable_overlay);
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_list(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // build liist
    ui.div_rel(area.end(""), (0., 100.), (0., 15.), add_lobby_list_title);
    ui.div_rel(area.end(""), (70., 75.), (10., 15.), add_lobby_list_refresh_indicator);
    ui.div_rel(area.end(""), (75., 90.), (10., 15.), add_lobby_list_refresh_button);
    ui.div_rel(area.end(""), (10., 90.), (15., 75.), add_lobby_list_subsection);
    ui.div_rel(area.end(""), (10., 90.), (75., 80.), add_navigation_subsection);
    ui.div_rel(area.end(""), (0., 100.), (80., 95.), add_new_lobby_button);

    // prepare windows
    // - these are defined with respect to the ui root, so we don't pass in area widgets
    ui.div(add_join_lobby_window);
    ui.div(add_make_lobby_window);

    // initialize UI
    ui.commands()
        .react()
        .trigger_resource_mutation::<LobbyPage>();
    ui.commands()
        .react()
        .trigger_resource_mutation::<LobbyPageRequest>();
}

// TODO: make lobby button needs to set MakeLobbyData to is_constructing=true

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiLobbyListPlugin;

impl Plugin for UiLobbyListPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(UiJoinLobbyWindowPlugin)
            .add_plugins(UiMakeLobbyWindowPlugin);
    }
}

//-------------------------------------------------------------------------------------------------------------------
