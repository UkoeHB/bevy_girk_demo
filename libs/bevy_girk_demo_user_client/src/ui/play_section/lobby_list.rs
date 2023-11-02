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

        // update entry text
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

fn open_join_lobby_window(
    In(lobby_index) : In<usize>,
    mut rcommands   : ReactCommands,
    lobby_page      : Res<ReactRes<LobbyPage>>,
){
    // get lobby id of lobby to join
    let Some(lobby_contents) = lobby_page.get().get(lobby_index)
    else { tracing::error!(lobby_index, "failed accessing lobby contents for join lobby attempt"); return; };

    //todo: open join lobby window w/ lobby contents
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn open_new_lobby_window(
    mut rcommands: ReactCommands,
){
    //todo: open new lobby window
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_title(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // title text
    let text = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 20., y: 40. },
                relative_2: Vec2{ x: 80., y: 70. },
                ..Default::default()
            }
        ).unwrap();

    let text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : MISC_FONT_COLOR,
        };

    rcommands.commands().spawn(
            TextElementBundle::new(
                text,
                TextParams::center()
                    .with_style(&text_style)
                    .with_height(Some(100.)),
                "Lobby List"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_indicator(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    let overlay = make_overlay(ui, &area, "", false);

    // indicator text
    let text = Widget::create(
            ui,
            overlay.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 5., y: 15. },
                relative_2: Vec2{ x: 98., y: 95. },
                ..Default::default()
            }
        ).unwrap();

    let text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : MISC_FONT_COLOR,
        };

    rcommands.commands().spawn(
            TextElementBundle::new(
                text,
                TextParams::centerright()
                    .with_style(&text_style)
                    .with_height(Some(40.)),
                "Loading..."
            )
        );

    // setup reactors
    rcommands.commands().add(
            move |world: &mut World| syscall(world, overlay, setup_refresh_indicator_reactors)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_refresh_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // overlay
    let button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 5., y: 10. },
                relative_2: Vec2{ x: 98., y: 90. },
                ..Default::default()
            }
        ).unwrap();

    // button ui
    let button_overlay = make_overlay(ui, &button_area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Refresh",
            |world| syscall(world, (), refresh_lobby_list)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_subsection(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // list box
    rcommands.commands().spawn(
            ImageElementBundle::new(
                    area,
                    ImageParams::center()
                        .with_depth(50.)
                        .with_width(Some(100.))
                        .with_height(Some(100.)),
                    asset_server.load(BOX),
                    Vec2::new(236.0, 139.0)
                )
        );

    // prepare contents
    let mut contents = Vec::with_capacity(LOBBY_LIST_SIZE as usize);

    let entry_height = (1. / LOBBY_LIST_SIZE as f32) * 95.;

    for i in 0..LOBBY_LIST_SIZE
    {
        let content_widget = make_overlay(ui, area, "", true);

        // text
        let y_start = 2.5 + (i       as f32)*entry_height;
        let y_end   = 2.5 + ((i + 1) as f32)*entry_height;

        let text = Widget::create(
                ui,
                content_widget.end(""),
                RelativeLayout{  //center
                    relative_1: Vec2{ x: 10., y: y_start },
                    relative_2: Vec2{ x: 78., y: y_end },
                    ..Default::default()
                }
            ).unwrap();

        let text_style = TextStyle {
                font      : asset_server.load(MISC_FONT),
                font_size : 45.0,
                color     : LOBBY_DISPLAY_FONT_COLOR,
            };

        let content_entity = rcommands.commands().spawn(
                TextElementBundle::new(
                    text,
                    TextParams::centerleft()
                        .with_style(&text_style)
                        .with_width(Some(100.))
                        .with_height(Some(70.)),
                    "Id: ??????, Owner: ??????, Players: 00/00, Watchers: 00/00"
                )
            ).id();

        // button
        let join_button_area = Widget::create(
                ui,
                content_widget.end(""),
                RelativeLayout{  //center
                    relative_1: Vec2{ x: 80., y: y_start + 0.5 },
                    relative_2: Vec2{ x: 98., y: y_end - 0.5 },
                    ..Default::default()
                }
            ).unwrap();

        make_basic_button(
                rcommands.commands(),
                asset_server,
                ui,
                &join_button_area,
                content_entity,
                "Join",
                move |world| syscall(world, i, open_join_lobby_window)
            );

        // save this entry
        contents.push((content_entity, content_widget));
    }

    let contents = Arc::new(contents);

    // update contents when lobby page changes
    rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World| syscall(world, contents.clone(), update_lobby_list_contents)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_list_stats(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    _ui          : &mut UiTree,
    area         : &Widget,
){
    // stats text
    let text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : LOBBY_LIST_STATS_FONT_COLOR,
        };

    let text_entity = rcommands.commands().spawn(
            TextElementBundle::new(
                area,
                TextParams::center()
                    .with_style(&text_style)
                    .with_width(Some(100.)),
                "(????-???? / ????)"
            )
        ).id();

    // update stats when lobby page updates
    rcommands.add_resource_mutation_reactor::<LobbyPage>(
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

fn add_clamp_now_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // button ui
    // - request the most recent possible lobby
    let button_overlay = make_overlay(ui, area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Now",
            move |world| syscall(world, (), request_lobby_list_now)
        );

    // block button and grey-out text when displaying 'now'
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyPageRequest>(
            move |world: &mut World|
            {
                let enable = !world.resource::<ReactRes<LobbyPageRequest>>().is_now();
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_left_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // button ui
    // - request the next most recent lobby
    let button_overlay = make_overlay(ui, area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "<",
            move |world| syscall(world, (), request_lobby_list_next_newer)
        );

    // block button and grey-out text when no newer lobbies to request
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World|
            {
                let (first, _, _) = world.resource::<ReactRes<LobbyPage>>().stats();
                let enable = first != 1;
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_right_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // button ui
    // - request the next older lobby
    let button_overlay = make_overlay(ui, area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            ">",
            move |world| syscall(world, (), request_lobby_list_next_older)
        );

    // block button and grey-out text when no older lobbies to request
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World|
            {
                let (_, last, total) = world.resource::<ReactRes<LobbyPage>>().stats();
                let enable = last != total;
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_oldest_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // button ui
    // - request the oldest lobbies
    let button_overlay = make_overlay(ui, area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Oldest",
            move |world| syscall(world, (), request_lobby_list_oldest)
        );

    // block button and grey-out text when last requested the oldest lobbies
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyPageRequest>(
            move |world: &mut World|
            {
                let enable = !world.resource::<ReactRes<LobbyPageRequest>>().is_oldest();
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_navigation_subsection(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // text displaying lobby list position
    let lobby_list_stats_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //center
                relative_1: Vec2{ x: 38., y: 20. },
                relative_2: Vec2{ x: 62., y: 60. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_stats(rcommands, asset_server, ui, &lobby_list_stats_area);

    // button: go to first page
    let clamp_left_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //far left
                relative_1: Vec2{ x: 1., y: 5. },
                relative_2: Vec2{ x: 13., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_clamp_now_button(rcommands, asset_server, ui, &clamp_left_button_area);

    // button: paginate left
    let paginate_left_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //mid left
                relative_1: Vec2{ x: 14., y: 5. },
                relative_2: Vec2{ x: 26., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_paginate_left_button(rcommands, asset_server, ui, &paginate_left_button_area);

    // button: paginate right
    let paginate_right_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //mid right
                relative_1: Vec2{ x: 74., y: 5. },
                relative_2: Vec2{ x: 86., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_paginate_right_button(rcommands, asset_server, ui, &paginate_right_button_area);

    // button: go to last page
    let clamp_right_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //far right
                relative_1: Vec2{ x: 87., y: 5. },
                relative_2: Vec2{ x: 99., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_clamp_oldest_button(rcommands, asset_server, ui, &clamp_right_button_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_new_lobby_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // overlay
    let button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 25., y: 10. },
                relative_2: Vec2{ x: 75., y: 90. },
                ..Default::default()
            }
        ).unwrap();

    // button ui
    let button_overlay = make_overlay(ui, &button_area, "new_lobby", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "New Lobby",
            |world| syscall(world, (), open_new_lobby_window)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_list(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // title
    let lobby_list_title = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 15. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_title(rcommands, asset_server, ui, &lobby_list_title);

    // refresh indicator
    let lobby_list_refresh_indicator = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 70., y: 10. },
                relative_2: Vec2{ x: 75., y: 15. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_refresh_indicator(rcommands, asset_server, ui, &lobby_list_refresh_indicator);

    // refresh button
    let lobby_list_refresh_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 75., y: 10. },
                relative_2: Vec2{ x: 90., y: 15. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_refresh_button(rcommands, asset_server, ui, &lobby_list_refresh_button);

    // list subsection
    let lobby_list_subsection = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 15. },
                relative_2: Vec2{ x: 90., y: 75. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_subsection(rcommands, asset_server, ui, &lobby_list_subsection);

    // navigation subsection
    let navigation_subsection = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 75. },
                relative_2: Vec2{ x: 90., y: 80. },
                ..Default::default()
            }
        ).unwrap();
    add_navigation_subsection(rcommands, asset_server, ui, &navigation_subsection);

    // new lobby button
    let new_lobby_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 80. },
                relative_2: Vec2{ x: 100., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_new_lobby_button(rcommands, asset_server, ui, &new_lobby_button);

    // initialize UI listening to lobby page
    rcommands.trigger_resource_mutation::<LobbyPage>();
    rcommands.trigger_resource_mutation::<LobbyPageRequest>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyListPlugin(app: &mut App)
{
    let timer_configs = app.world.resource::<TimerConfigs>();
    let lobby_list_refresh = Duration::from_millis(timer_configs.lobby_list_refresh_ms);

    app
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
                .run_if(|play_section: Query<(), (With<Selected>, With<MainPlayButton>)>| !play_section.is_empty() )
                .run_if(resource_equals(ConnectionStatus::Connected))
                .run_if(on_timer(lobby_list_refresh)
                    .or_else(
                        |connection_status: Res<ConnectionStatus>| { connection_status.is_changed() }
                    )
                )
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
