//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
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
    lobby_page_req : Res<ReactRes<LobbyPageRequest>>,
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
    mut lobby_page_req : ResMut<ReactRes<LobbyPageRequest>>,
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
    mut lobby_page_req : ResMut<ReactRes<LobbyPageRequest>>,
    lobby_page         : Res<ReactRes<LobbyPage>>,
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
    mut lobby_page_req : ResMut<ReactRes<LobbyPageRequest>>,
    lobby_page         : Res<ReactRes<LobbyPage>>,
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
    mut lobby_page_req : ResMut<ReactRes<LobbyPageRequest>>,
){
    // do nothing if there is already a pending lobby search
    let Ok(target_entity) = lobby_search.get_single()
    else { tracing::debug!("ignoring lobby search request because a search is already pending"); return; };

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
    rcommands.add_entity_insertion_reactor::<PendingRequest>(
            lobby_search_entity,
            move |world: &mut World| syscall(world, (MainUI, [text_widget_clone.clone()], []), toggle_ui_visibility)
        );

    // deactivate text when there is not a pending request
    rcommands.add_entity_removal_reactor::<React<PendingRequest>>(
            lobby_search_entity,
            move |world: &mut World| syscall(world, (MainUI, [], [text_widget.clone()]), toggle_ui_visibility)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn update_lobby_list_contents(
    In(contents)   : In<Arc<Vec<(Entity, Widget)>>>,
    mut ui         : Query<&mut UiTree, With<MainUI>>,
    mut text_query : Query<&mut Text>,
    lobby_page     : Res<ReactRes<LobbyPage>>,
){
    // ui tree
    let mut ui = ui.single_mut();

    // lobby list entries
    let entries = lobby_page.get();

    // update contents
    for (idx, (content_entity, content_widget)) in contents.iter().enumerate()
    {
        // get entry text
        let Ok(mut text) = text_query.get_mut(*content_entity)
        else { tracing::error!("text entity is missing for lobby list contents"); return; };
        let text_section = &mut text.sections[0].value;

        // get lobby list entry
        let entry = entries.get(idx);

        // update entry visibility
        let Ok(widget_branch) = content_widget.fetch_mut(&mut ui) else { continue; };
        widget_branch.set_visibility(entry.is_some());

        // update entry text
        let Some(entry) = entry else { continue; };

        let _ = write!(
                text_section,
                "Id: {}, Owner: {}, Players: {}/{}, Watchers: {}/{}",
                entry.id % 1_000_000u64,
                entry.owner_id % 1_000_000u128,
                entry.num(ClickLobbyMemberType::Player),
                entry.max(ClickLobbyMemberType::Player),
                entry.num(ClickLobbyMemberType::Watcher),
                entry.max(ClickLobbyMemberType::Watcher),
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

fn add_lobby_list_title(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // title text
    let text = relative_widget(ctx, area.end(""), (20., 80.), (40., 70.));
    spawn_basic_text(
            ctx,
            text,
            MISC_FONT_COLOR,
            TextParams::center()
                .with_height(Some(100.)),
            "Lobby List"
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_indicator(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // make overlay so indicator visibility starts `false`
    let overlay = make_overlay(ctx.ui(), &area, "", false);

    // indicator text
    let text = relative_widget(ctx, overlay.end(""), (5., 98.), (15., 95.));
    spawn_basic_text(
            ctx,
            text,
            MISC_FONT_COLOR,
            TextParams::centerright()
                .with_height(Some(40.)),
            "Loading..."
        );

    // setup reactors
    ctx.commands().add(move |world: &mut World| syscall(world, overlay, setup_refresh_indicator_reactors));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_button(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // button ui
    let button_overlay = relative_widget(ctx, area.end(""), (5., 98.), (10., 90.));
    let button_entity  = ctx.commands().spawn_empty().id();

    make_basic_button(ctx, &button_overlay, button_entity, "Refresh", |world| syscall(world, (), refresh_lobby_list));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_subsection(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // list box
    spawn_plain_box(ctx, area.clone());

    // prepare contents
    let mut contents = Vec::with_capacity(LOBBY_LIST_SIZE as usize);

    let entry_height = (1. / LOBBY_LIST_SIZE as f32) * 95.;

    for i in 0..LOBBY_LIST_SIZE
    {
        let content_widget = make_overlay(ctx.ui(), area, "", true);

        // text
        let y_start = 2.5 + (i       as f32)*entry_height;
        let y_end   = 2.5 + ((i + 1) as f32)*entry_height;
 
        let text = relative_widget(ctx, content_widget.end(""), (10., 78.), (y_start, y_end));

        let content_entity = spawn_basic_text(
                ctx,
                text,
                LOBBY_DISPLAY_FONT_COLOR,
                TextParams::centerleft()
                    .with_width(Some(100.))
                    .with_height(Some(70.)),
                "Id: ??????, Owner: ??????, Players: 00/00, Watchers: 00/00"
            );

        // button
        let join_button_area = relative_widget(ctx, content_widget.end(""), (80., 98.), (y_start + 0.5, y_end - 0.5));

        make_basic_button(ctx, &join_button_area, content_entity, "Join",
                move |world| syscall(world, i, open_join_lobby_window)
            );

        // save this entry
        contents.push((content_entity, content_widget));
    }

    let contents = Arc::new(contents);

    // update contents when lobby page changes
    ctx.rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World| syscall(world, contents.clone(), update_lobby_list_contents)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_stats(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // stats text
    let text_entity = spawn_basic_text(
            ctx,
            area.clone(),
            LOBBY_LIST_STATS_FONT_COLOR,
            TextParams::center()
                .with_width(Some(100.)),
            "(????-???? / ????)"
        );

    // update stats when lobby page updates
    ctx.rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World|
            {
                // define updated text
                let (first, last, total) = world.resource::<ReactRes<LobbyPage>>().stats();

                let mut text_buffer = String::new();
                let _ = write!(text_buffer, "({}-{} / {})", first, last, total);

                // update UI text
                syscall(world, (text_entity, text_buffer), update_ui_text);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_now_button(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // button ui
    // - request the most recent possible lobby
    let button_entity = ctx.commands().spawn_empty().id();
    make_basic_button(ctx, &area, button_entity, "Now", |world| syscall(world, (), request_lobby_list_now));

    // disable button when displaying 'now'
    let disable_overlay = make_overlay(ctx.ui(), &area, "", false);
    ctx.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ctx.rcommands.add_resource_mutation_reactor::<LobbyPageRequest>(
            move |world: &mut World|
            {
                let enable = !world.resource::<ReactRes<LobbyPageRequest>>().is_now();
                syscall(world, (enable, disable_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_left_button(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // button ui
    // - request the next most recent lobby
    let button_entity = ctx.commands().spawn_empty().id();
    make_basic_button(ctx, &area, button_entity, "<", |world| syscall(world, (), request_lobby_list_next_newer));

    // disable button when no newer lobbies to request
    let disable_overlay = make_overlay(ctx.ui(), &area, "", false);
    ctx.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ctx.rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World|
            {
                let (first, _, _) = world.resource::<ReactRes<LobbyPage>>().stats();
                let enable = first != 1;
                syscall(world, (enable, disable_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_right_button(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // button ui
    // - request the next older lobby
    let button_entity = ctx.commands().spawn_empty().id();
    make_basic_button(ctx, &area, button_entity, ">", |world| syscall(world, (), request_lobby_list_next_older));

    // disable button when no older lobbies to request
    let disable_overlay = make_overlay(ctx.ui(), &area, "", false);
    ctx.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ctx.rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World|
            {
                let (_, last, total) = world.resource::<ReactRes<LobbyPage>>().stats();
                let enable = last != total;
                syscall(world, (enable, disable_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_oldest_button(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // button ui
    // - request the oldest lobbies
    let button_entity = ctx.commands().spawn_empty().id();
    make_basic_button(ctx, &area, button_entity, "Oldest", |world| syscall(world, (), request_lobby_list_oldest));

    // disable button when last requested the oldest lobbies
    let disable_overlay = make_overlay(ctx.ui(), &area, "", false);
    ctx.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ctx.rcommands.add_resource_mutation_reactor::<LobbyPageRequest>(
            move |world: &mut World|
            {
                let enable = !world.resource::<ReactRes<LobbyPageRequest>>().is_oldest();
                syscall(world, (enable, disable_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_navigation_subsection(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // text displaying lobby li (center)st position
    let lobby_list_stats_area = relative_widget(ctx, area.end(""), (38., 62.), (20., 60.));
    add_lobby_list_stats(ctx, &lobby_list_stats_area);

    // button: go to first page (far left)
    let clamp_left_button_area = relative_widget(ctx, area.end(""), (1., 13.), (5., 95.));
    add_clamp_now_button(ctx, &clamp_left_button_area);

    // button: paginate left (mid left)
    let paginate_left_button_area = relative_widget(ctx, area.end(""), (14., 26.), (5., 95.));
    add_paginate_left_button(ctx, &paginate_left_button_area);

    // button: paginate right (mid right)
    let paginate_right_button_area = relative_widget(ctx, area.end(""), (74., 86.), (5., 95.));
    add_paginate_right_button(ctx, &paginate_right_button_area);

    // button: go to last page (far right)
    let clamp_right_button_area = relative_widget(ctx, area.end(""), (87., 99.), (5., 95.));
    add_clamp_oldest_button(ctx, &clamp_right_button_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_new_lobby_button(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // overlay
    let button_area = relative_widget(ctx, area.end(""), (25., 75.), (10., 90.));

    // button ui
    let button_overlay = make_overlay(ctx.ui(), &button_area, "", true);
    let button_entity  = ctx.commands().spawn_empty().id();

    make_basic_button(ctx, &button_overlay, button_entity, "New Lobby", |world| syscall(world, (), open_make_lobby_window));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_list(ctx: &mut UiBuilderCtx, area: &Widget)
{
    // title
    let lobby_list_title = relative_widget(ctx, area.end(""), (0., 100.), ( 0., 15.));
    add_lobby_list_title(ctx, &lobby_list_title);

    // refresh indicator
    let lobby_list_refresh_indicator = relative_widget(ctx, area.end(""), (70., 75.), (10., 15.));
    add_lobby_list_refresh_indicator(ctx, &lobby_list_refresh_indicator);

    // refresh button
    let lobby_list_refresh_button = relative_widget(ctx, area.end(""), (75., 90.), (10., 15.));
    add_lobby_list_refresh_button(ctx, &lobby_list_refresh_button);

    // list subsection
    let lobby_list_subsection = relative_widget(ctx, area.end(""), (10., 90.), (15., 75.));
    add_lobby_list_subsection(ctx, &lobby_list_subsection);

    // navigation subsection
    let navigation_subsection = relative_widget(ctx, area.end(""), (10., 90.), (75., 80.));
    add_navigation_subsection(ctx, &navigation_subsection);

    // new lobby button
    let new_lobby_button = relative_widget(ctx, area.end(""), (0., 100.), (80., 95.));
    add_new_lobby_button(ctx, &new_lobby_button);


    // prepare windows
    // - these are defined with respect to the ui root, so we don't pass in area widgets
    add_join_lobby_window(ctx);
    add_make_lobby_window(ctx);


    // initialize UI listening to lobby page
    ctx.rcommands.trigger_resource_mutation::<LobbyPage>();
    ctx.rcommands.trigger_resource_mutation::<LobbyPageRequest>();
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
                // - on timer OR just connected to host (note: test timer first to avoid double-refresh when timer
                //   is saturated)
                .run_if(|play_section: Query<(), (With<Selected>, With<MainPlayButton>)>| !play_section.is_empty())
                .run_if(|status: Res<ReactRes<ConnectionStatus>>| **status == ConnectionStatus::Connected)
                .run_if(on_timer(lobby_list_refresh)
                    .or_else(|status: Res<ReactRes<ConnectionStatus>>| status.is_changed())
                )
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
