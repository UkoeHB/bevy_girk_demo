//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn refresh_lobby_list(
    mut rcommands  : ReactCommands,
    client         : Res<HostUserClient>,
    lobby_search   : Query<Entity, (With<LobbySearch>, Without<React<PendingRequest>>)>,
    lobby_page_req : ReactRes<LobbyPageRequest>,
){
    // do nothing if there is already a pending lobby search
    let Ok(target_entity) = lobby_search.get_single()
    else { tracing::debug!("ignoring lobby search request because a search is already pending"); return; };

    // re-request the last-requested lobby page
    tracing::trace!("refreshing lobby list");
    rerequest_latest_lobby_page(&mut rcommands, &client, target_entity, &lobby_page_req);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn request_lobby_list_now(
    mut rcommands      : ReactCommands,
    client             : Res<HostUserClient>,
    lobby_search       : Query<Entity, (With<LobbySearch>, Without<React<PendingRequest>>)>,
    mut lobby_page_req : ReactResMut<LobbyPageRequest>,
){
    // do nothing if there is already a pending lobby search
    let Ok(target_entity) = lobby_search.get_single()
    else { tracing::debug!("ignoring lobby search request because a search is already pending"); return; };

    // make request
    // - we request the highest-possible lobby id in order to get the youngest available lobby
    let req = LobbySearchRequest::PageOlder{ youngest_id: u64::MAX, num: LOBBY_LIST_SIZE as u16 };

    // send request
    tracing::trace!("requesting lobby list: now");
    let Ok(new_req) = client.request(UserToHostRequest::LobbySearch(req.clone())) else { return; };

    // save request
    lobby_page_req.get_mut(&mut rcommands).set(req);
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn request_lobby_list_next_newer(
    mut rcommands      : ReactCommands,
    client             : Res<HostUserClient>,
    lobby_search       : Query<Entity, (With<LobbySearch>, Without<React<PendingRequest>>)>,
    mut lobby_page_req : ReactResMut<LobbyPageRequest>,
    lobby_page         : ReactRes<LobbyPage>,
){
    // do nothing if there is already a pending lobby search
    let Ok(target_entity) = lobby_search.get_single()
    else { tracing::debug!("ignoring lobby search request because a search is already pending"); return; };

    // make request
    let oldest_id = lobby_page
        .get()
        .get(0)  //youngest currently-displayed lobby
        .map_or(
            // this branch may execute if lobbies in the last requested page are all removed by server updates, or
            // if the returned page is empty
            match lobby_page_req.get()
            {
                LobbySearchRequest::LobbyId(id) => id.saturating_sub(1u64),
                LobbySearchRequest::PageNewer{ oldest_id, num } =>
                {
                    oldest_id.saturating_add(*num as u64)
                }
                LobbySearchRequest::PageOlder{ youngest_id, num: _ } =>
                {
                    youngest_id.saturating_add(1u64)
                }
            },
            |contents| contents.id.saturating_add(1u64),  //next page starts at lobby younger than our current youngest
        );

    let req = LobbySearchRequest::PageNewer{
            oldest_id,
            num: LOBBY_LIST_SIZE as u16
        };

    // send request
    tracing::trace!("requesting lobby list: next newer");
    let Ok(new_req) = client.request(UserToHostRequest::LobbySearch(req.clone())) else { return; };

    // save request
    lobby_page_req.get_mut(&mut rcommands).set(req);
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn request_lobby_list_next_older(
    mut rcommands      : ReactCommands,
    client             : Res<HostUserClient>,
    lobby_search       : Query<Entity, (With<LobbySearch>, Without<React<PendingRequest>>)>,
    mut lobby_page_req : ReactResMut<LobbyPageRequest>,
    lobby_page         : ReactRes<LobbyPage>,
){
    // do nothing if there is already a pending lobby search
    let Ok(target_entity) = lobby_search.get_single()
    else { tracing::debug!("ignoring lobby search request because a search is already pending"); return; };

    // make request
    let youngest_id = lobby_page
        .get()
        .get(lobby_page.len().saturating_sub(1))  //oldest currently-displayed lobby
        .map_or(
            // this branch may execute if lobbies in the last requested page are all removed by server updates, or
            // if the returned page is empty
            match lobby_page_req.get()
            {
                LobbySearchRequest::LobbyId(id) => id.saturating_sub(1u64),
                LobbySearchRequest::PageNewer{ oldest_id, num: _ } =>
                {
                    oldest_id.saturating_sub(1u64)
                }
                LobbySearchRequest::PageOlder{ youngest_id, num } =>
                {
                    youngest_id.saturating_sub(*num as u64)
                }
            },
            |contents| contents.id.saturating_sub(1u64),  //next page starts at lobby older than our current oldest
        );

    let req = LobbySearchRequest::PageOlder{
            youngest_id,
            num: LOBBY_LIST_SIZE as u16
        };

    // send request
    tracing::trace!("requesting lobby list: next older");
    let Ok(new_req) = client.request(UserToHostRequest::LobbySearch(req.clone())) else { return; };

    // save request
    lobby_page_req.get_mut(&mut rcommands).set(req);
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn request_lobby_list_oldest(
    mut rcommands      : ReactCommands,
    client             : Res<HostUserClient>,
    lobby_search       : Query<Entity, (With<LobbySearch>, Without<React<PendingRequest>>)>,
    mut lobby_page_req : ReactResMut<LobbyPageRequest>,
){
    // do nothing if there is already a pending lobby search
    let Ok(target_entity) = lobby_search.get_single()
    else { tracing::warn!("ignoring lobby search request because a request is already pending"); return; };

    // make request
    // - we request the lowest-possible lobby id in order to get the oldest available lobby
    let req = LobbySearchRequest::PageNewer{ oldest_id: 0u64, num: LOBBY_LIST_SIZE as u16 };

    // send request
    tracing::trace!("requesting lobby list: oldest");
    let Ok(new_req) = client.request(UserToHostRequest::LobbySearch(req.clone())) else { return; };

    // save request
    lobby_page_req.get_mut(&mut rcommands).set(req);
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_refresh_indicator_reactors(
    In(text_widget) : In<Widget>,
    mut rcommands   : ReactCommands,
    lobby_search    : Query<Entity, With<LobbySearch>>,
){
    let lobby_search_entity = lobby_search.single();

    // activate text when there is a pending request
    let text_widget_clone = text_widget.clone();
    rcommands.on(entity_insertion::<PendingRequest>(lobby_search_entity),
            move |mut ui: UiUtils<MainUI>| ui.toggle(true, &text_widget_clone)
        );

    // deactivate text when there is not a pending request
    rcommands.on(entity_removal::<PendingRequest>(lobby_search_entity),
            move |mut ui: UiUtils<MainUI>| ui.toggle(false, &text_widget)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn update_lobby_list_contents(
    In(contents) : In<Arc<Vec<(Entity, Widget)>>>,
    mut ui       : UiUtils<MainUI>,
    lobby_page   : ReactRes<LobbyPage>,
){
    // lobby list entries
    let entries = lobby_page.get();

    // update contents
    for (idx, (content_entity, content_widget)) in contents.iter().enumerate()
    {
        // get lobby list entry
        let entry = entries.get(idx);

        // update entry visibility
        ui.toggle(entry.is_some(), &content_widget);

        // update entry text
        let Some(entry) = entry else { continue; };
        let _ = ui.text.write(*content_entity, 0,
                |text|
                {
                    write!(
                            text,
                            "Id: {}, Owner: {}, Players: {}/{}, Watchers: {}/{}",
                            entry.id % 1_000_000u64,
                            entry.owner_id % 1_000_000u128,
                            entry.num(ClickLobbyMemberType::Player),
                            entry.max(ClickLobbyMemberType::Player),
                            entry.num(ClickLobbyMemberType::Watcher),
                            entry.max(ClickLobbyMemberType::Watcher),
                        )
                }
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn open_join_lobby_window(In(lobby_list_index): In<usize>, mut rcommands: ReactCommands)
{
    // activate the window
    rcommands.send(ActivateJoinLobbyWindow{ lobby_list_index });
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn open_make_lobby_window(mut rcommands: ReactCommands)
{
    // activate the window
    rcommands.send(ActivateMakeLobbyWindow);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_title(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // title text
    let text = relative_widget(ui.tree(), area.end(""), (20., 80.), (40., 70.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(100.)), "Lobby List");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_indicator(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // make overlay so indicator visibility starts `false`
    let overlay = make_overlay(ui.tree(), &area, "", false);

    // set text style
    ui.add_style(basic_text_default_light_style());

    // indicator text
    let text = relative_widget(ui.tree(), overlay.end(""), (5., 98.), (15., 95.));
    spawn_basic_text(ui, text, TextParams::centerright().with_height(Some(40.)), "Loading...");

    // setup reactors
    ui.commands().add(move |world: &mut World| syscall(world, overlay, setup_refresh_indicator_reactors));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    let button_overlay = relative_widget(ui.tree(), area.end(""), (5., 98.), (10., 90.));
    spawn_basic_button(ui, &button_overlay, "Refresh", refresh_lobby_list);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_subsection(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // list box
    spawn_plain_box(ui, area.clone(), None);

    // prepare contents
    let mut contents = Vec::with_capacity(LOBBY_LIST_SIZE as usize);

    let entry_height = (1. / LOBBY_LIST_SIZE as f32) * 95.;

    for i in 0..LOBBY_LIST_SIZE
    {
        let content_widget = make_overlay(ui.tree(), area, "", true);

        // text
        let y_start = 2.5 + (i       as f32)*entry_height;
        let y_end   = 2.5 + ((i + 1) as f32)*entry_height;
 
        let text = relative_widget(ui.tree(), content_widget.end(""), (5., 78.), (y_start, y_end));

        let content_entity = spawn_basic_text(
                ui,
                text,
                TextParams::centerleft()
                    .with_width(Some(100.)),
                "Id: ??????, Owner: ??????, Players: 00/00, Watchers: 00/00"
            );

        // button
        let join_button_area = relative_widget(ui.tree(), content_widget.end(""), (80., 98.), (y_start + 0.5, y_end - 0.5));
        spawn_basic_button(ui, &join_button_area, "Join", prep_syscall(i, open_join_lobby_window));

        // save this entry
        contents.push((content_entity, content_widget));
    }

    let contents = Arc::new(contents);

    // update contents when lobby page changes
    ui.rcommands.on(resource_mutation::<LobbyPage>(), prep_syscall(contents, update_lobby_list_contents));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_stats(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // stats text
    let text_entity = spawn_basic_text(ui, area.clone(), TextParams::center().with_width(Some(100.)), "(????-???? / ????)");

    // update stats when lobby page updates
    ui.rcommands.on(resource_mutation::<LobbyPage>(),
            move |mut ui: UiUtils<MainUI>, page: ReactRes<LobbyPage>|
            {
                let (first, last, total) = page.stats();
                ui.text.write(text_entity, 0, |text| write!(text, "({}-{} / {})", first, last, total)).unwrap();
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_now_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    // - request the most recent possible lobby
    let button_entity = spawn_basic_button(ui, &area, "Now", request_lobby_list_now);

    // disable button when displaying 'now'
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyPageRequest>(),
            move |mut ui: UiUtils<MainUI>, page_req: ReactRes<LobbyPageRequest>|
            {
                let enable = !page_req.is_now();
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_left_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    // - request the next most recent lobby
    let button_entity = spawn_basic_button(ui, &area, "<", request_lobby_list_next_newer);

    // disable button when no newer lobbies to request
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyPage>(),
            move |mut ui: UiUtils<MainUI>, page: ReactRes<LobbyPage>|
            {
                let (first, _, _) = page.stats();
                let enable = first != 1;
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_right_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    // - request the next older lobby
    let button_entity = spawn_basic_button(ui, &area, ">", request_lobby_list_next_older);

    // disable button when no older lobbies to request
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyPage>(),
            move |mut ui: UiUtils<MainUI>, page: ReactRes<LobbyPage>|
            {
                let (_, last, total) = page.stats();
                let enable = last != total;
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_oldest_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    // - request the oldest lobbies
    let button_entity = spawn_basic_button(ui, &area, "Oldest", request_lobby_list_oldest);

    // disable button when last requested the oldest lobbies
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyPageRequest>(),
            move |mut ui: UiUtils<MainUI>, page_req: ReactRes<LobbyPageRequest>|
            {
                let enable = !page_req.is_oldest();
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_navigation_subsection(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // text displaying lobby li (center)st position
    let lobby_list_stats_area = relative_widget(ui.tree(), area.end(""), (38., 62.), (20., 60.));
    ui.div(|ui| add_lobby_list_stats(ui, &lobby_list_stats_area));

    // button: go to first page (far left)
    let clamp_left_button_area = relative_widget(ui.tree(), area.end(""), (1., 13.), (5., 95.));
    ui.div(|ui| add_clamp_now_button(ui, &clamp_left_button_area));

    // button: paginate left (mid left)
    let paginate_left_button_area = relative_widget(ui.tree(), area.end(""), (14., 26.), (5., 95.));
    ui.div(|ui| add_paginate_left_button(ui, &paginate_left_button_area));

    // button: paginate right (mid right)
    let paginate_right_button_area = relative_widget(ui.tree(), area.end(""), (74., 86.), (5., 95.));
    ui.div(|ui| add_paginate_right_button(ui, &paginate_right_button_area));

    // button: go to last page (far right)
    let clamp_right_button_area = relative_widget(ui.tree(), area.end(""), (87., 99.), (5., 95.));
    ui.div(|ui| add_clamp_oldest_button(ui, &clamp_right_button_area));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_new_lobby_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // overlay
    let button_area = relative_widget(ui.tree(), area.end(""), (25., 75.), (10., 90.));

    // button ui
    let button_overlay = make_overlay(ui.tree(), &button_area, "", true);
    let button_entity  = spawn_basic_button(ui, &button_overlay, "New Lobby", open_make_lobby_window);

    // disable button when we are in a lobby already
    let disable_overlay = make_overlay(ui.tree(), &button_overlay, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move |mut ui: UiUtils<MainUI>, display: ReactRes<LobbyDisplay>|
            {
                let enable = !display.is_set();
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_list(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // title
    let lobby_list_title = relative_widget(ui.tree(), area.end(""), (0., 100.), ( 0., 15.));
    ui.div(|ui| add_lobby_list_title(ui, &lobby_list_title));

    // refresh indicator
    let lobby_list_refresh_indicator = relative_widget(ui.tree(), area.end(""), (70., 75.), (10., 15.));
    ui.div(|ui| add_lobby_list_refresh_indicator(ui, &lobby_list_refresh_indicator));

    // refresh button
    let lobby_list_refresh_button = relative_widget(ui.tree(), area.end(""), (75., 90.), (10., 15.));
    ui.div(|ui| add_lobby_list_refresh_button(ui, &lobby_list_refresh_button));

    // list subsection
    let lobby_list_subsection = relative_widget(ui.tree(), area.end(""), (10., 90.), (15., 75.));
    ui.div(|ui| add_lobby_list_subsection(ui, &lobby_list_subsection));

    // navigation subsection
    let navigation_subsection = relative_widget(ui.tree(), area.end(""), (10., 90.), (75., 80.));
    ui.div(|ui| add_navigation_subsection(ui, &navigation_subsection));

    // new lobby button
    let new_lobby_button = relative_widget(ui.tree(), area.end(""), (0., 100.), (80., 95.));
    ui.div(|ui| add_new_lobby_button(ui, &new_lobby_button));


    // prepare windows
    // - these are defined with respect to the ui root, so we don't pass in area widgets
    ui.div(|ui| add_join_lobby_window(ui));
    ui.div(|ui| add_make_lobby_window(ui));


    // initialize UI listening to lobby page
    ui.rcommands.trigger_resource_mutation::<LobbyPage>();
    ui.rcommands.trigger_resource_mutation::<LobbyPageRequest>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyListPlugin(app: &mut App)
{
    let timer_configs = app.world.resource::<TimerConfigs>();
    let lobby_list_refresh = Duration::from_millis(timer_configs.lobby_list_refresh_ms);

    app
        .add_plugins(UiJoinLobbyWindowPlugin)
        .add_plugins(UiMakeLobbyWindowPlugin)
        .add_systems(PreUpdate,
            (
                refresh_lobby_list, apply_deferred,
            ).chain()
                // refresh the list automatically if:
                // - in play section
                // - connected to host
                //todo: not in game
                // - on timer
                //   OR just connected to host (note: test timer first to avoid double-refresh when timer
                //      is saturated)
                //   OR the lobby display was just changed
                //   OR the user just toggled to the play section
                .run_if(|play_section: Query<(), (With<Selected>, With<MainPlayButton>)>| !play_section.is_empty())
                .run_if(|status: ReactRes<ConnectionStatus>| *status == ConnectionStatus::Connected)
                .run_if(on_timer(lobby_list_refresh)
                    .or_else(|status: ReactRes<ConnectionStatus>| status.is_changed())
                    .or_else(|display: ReactRes<LobbyDisplay>| display.is_changed())
                    .or_else(|play_section: Query<(), (Added<Selected>, With<MainPlayButton>)>| !play_section.is_empty())
                )
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
