//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::{*, syscall};
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;
use std::sync::Arc;
use std::vec::Vec;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

trait ListPageTrait: Send + Sync + 'static
{
    fn set(&mut self, page: usize);
    fn get(&self) -> usize;
    fn member_type() -> ClickLobbyMemberType;
}

/// Tracks the current player list page. Starts at 0.
#[derive(Resource, Debug)]
struct PlayerListPage(usize);

impl ListPageTrait for PlayerListPage
{
    fn set(&mut self, page: usize) { self.0 = page; }
    fn get(&self) -> usize { self.0 }
    fn member_type() -> ClickLobbyMemberType { ClickLobbyMemberType::Player }
}

/// Tracks the current watcher list page. Starts at 0.
#[derive(Resource, Debug)]
struct WatcherListPage(usize);

impl ListPageTrait for WatcherListPage
{
    fn set(&mut self, page: usize) { self.0 = page; }
    fn get(&self) -> usize { self.0 }
    fn member_type() -> ClickLobbyMemberType { ClickLobbyMemberType::Watcher }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn max_page(lobby_contents: &ClickLobbyContents, member_type: ClickLobbyMemberType) -> usize
{
    let mut num_pages        = lobby_contents.max(member_type) as usize / NUM_LOBBY_CONTENT_ENTRIES;
    let num_dangling_entries = lobby_contents.max(member_type) as usize % NUM_LOBBY_CONTENT_ENTRIES;

    if num_dangling_entries == 0 { num_pages = num_pages.saturating_sub(1); }

    num_pages
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn page_is_maxed<ListPage: ListPageTrait>(lobby: &LobbyDisplay, list_page: &ListPage) -> bool
{
    let Some(lobby_contents) = lobby.get() else { return true; };
    list_page.get() >= max_page(lobby_contents, ListPage::member_type())
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn leave_current_lobby(
    client : Res<HostUserClient>,
    lobby  : Res<ReactRes<LobbyDisplay>>,
){
    let Some(lobby_data) = lobby.get()
    else { tracing::error!("tried to leave lobby but we aren't in a lobby"); return; };

    if let Err(err) = client.send(UserToHostMsg::LeaveLobby{ id: lobby_data.id })
    { tracing::warn!(?err, lobby_data.id, "failed sending leave lobby message to host server"); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn start_current_lobby(
    client : Res<HostUserClient>,
    lobby  : Res<ReactRes<LobbyDisplay>>,
){
    let Some(lobby_data) = lobby.get()
    else { tracing::error!("tried to start lobby but we aren't in a lobby"); return; };

    if let Err(err) = client.send(UserToHostMsg::LaunchLobbyGame{ id: lobby_data.id })
    { tracing::warn!(?err, lobby_data.id, "failed sending launch lobby message to host server"); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn update_list_contents<ListPage: ListPageTrait>(
    In((
        content_entities,
        new_page,
    ))                   : In<(Arc<Vec<Entity>>, usize)>,
    mut rcommands        : ReactCommands,
    mut text_query       : Query<&mut Text>,
    lobby                : Res<ReactRes<LobbyDisplay>>,
    mut member_list_page : ResMut<ReactRes<ListPage>>,
){
    // get list index for first element
    let first_idx = new_page * NUM_LOBBY_CONTENT_ENTRIES;

    // list-type tag
    let member_type = ListPage::member_type();
    let tag = match member_type
    {
        ClickLobbyMemberType::Player  => "Player",
        ClickLobbyMemberType::Watcher => "Watcher",
    };

    // update contents
    for (idx, content_entity) in content_entities.iter().enumerate()
    {
        let member_idx    = first_idx + idx;
        let member_number = first_idx + idx + 1;

        // clear entry
        let Ok(mut text) = text_query.get_mut(*content_entity)
        else { tracing::error!("text entity is missing for list contents"); return; };
        let text_section = &mut text.sections[0].value;
        text_section.clear();

        // check if the entry corresponds to a member slot
        let Some(lobby_contents) = lobby.get() else { continue; };
        if member_number > lobby_contents.max(member_type) as usize { continue; }

        // update entry text
        match lobby_contents.get_id(member_type, member_idx)
        {
            None =>
            {
                let _ = write!(text_section, "{} {}:", tag, member_number);
            }
            Some(member_id) =>
            {
                let _ = write!(text_section, "{} {}: {}", tag, member_number, member_id % 1_000_000u128);
            }
        }
    }

    // update the list page
    // - we mutate this even if the value won't change in case reactors need to chain off lobby display changes
    // - we clamp the new page value so we don't show an empty page other than the 0th page
    let new_page = match lobby.get()
    {
        None                 => 0,
        Some(lobby_contents) =>
        {
            match first_idx < lobby_contents.max(member_type) as usize
            {
                true  => new_page,
                false => max_page(&lobby_contents, member_type),
            }
        }
    };
    member_list_page.get_mut(&mut rcommands).set(new_page);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Update list contents when the lobby display is mutated.
/// - We use a generic over the list page type so the local here is unique per list type.
fn update_display_list_contents_on_lobby_display<ListPage: ListPageTrait>(
    In(content_entities) : In<Arc<Vec<Entity>>>,
    mut last_lobby_id    : Local<Option<u64>>,
    mut commands         : Commands,
    lobby                : Res<ReactRes<LobbyDisplay>>,
    member_list_page     : Res<ReactRes<ListPage>>,
){
    // get next list page
    // - we reset to 0 when the lobby changes
    // - only change the page if the lobby id has changed
    let current_lobby_id = lobby.lobby_id();

    let next_list_page = match *last_lobby_id == current_lobby_id
    {
        true  => member_list_page.get(),
        false => 0,
    };

    // update the recorded lobby id
    *last_lobby_id = current_lobby_id;

    // update the list contents
    // note: it would be more efficient to use world access directly here, but you can't have Locals in exclusive systems
    commands.add(
            move |world: &mut World| syscall(world, (content_entities, next_list_page), update_list_contents::<ListPage>)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_title(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    let display_title_text = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 20., y: 40. },
                relative_2: Vec2{ x: 80., y: 70. },
                ..Default::default()
            }
        ).unwrap();

    let display_title_text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : MISC_FONT_COLOR,
        };

    rcommands.commands().spawn(
            TextElementBundle::new(
                display_title_text,
                TextParams::center()
                    .with_style(&display_title_text_style)
                    .with_height(Some(100.)),
                "Current Lobby"
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_summary_box(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // add text to summary box
    let summary_box_text = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //center
                relative_1: Vec2{ x: 5., y: 15. },
                relative_2: Vec2{ x: 95., y: 97.5 },
                ..Default::default()
            }
        ).unwrap();

    let summary_box_text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : LOBBY_DISPLAY_FONT_COLOR,
        };

    let default_text = "Lobby: ?????? -- Owner: ??????";
    let text_entity = rcommands.commands().spawn(
            TextElementBundle::new(
                summary_box_text,
                TextParams::center()
                    .with_style(&summary_box_text_style)
                    .with_depth(100.)
                    .with_width(Some(90.)),
                default_text
            )
        ).id();

    // update the text when the lobby display changes
    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            {
                // define updated text
                let mut text_buffer = String::from(default_text);
                match world.resource::<ReactRes<LobbyDisplay>>().get()
                {
                    Some(lobby_contents) =>
                    {
                        let _ = write!(
                                text_buffer,
                                "Lobby: {} -- Owner: {}",
                                lobby_contents.id % 1_000_000u64,
                                lobby_contents.owner_id % 1_000_000u128
                            );
                    }
                    None => ()
                };

                // update UI text
                syscall(world, (text_entity, text_buffer), update_ui_text);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_header<ListPage: ListPageTrait>(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // outline for header
    let header_overlay = make_overlay(ui, &area, "", true);
    rcommands.commands().spawn(
            ImageElementBundle::new(
                header_overlay,
                    ImageParams::center()
                        .with_depth(50.1)
                        .with_width(Some(100.))
                        .with_height(Some(100.)),
                    asset_server.load(OUTLINE),
                    Vec2::new(236.0, 139.0)
                )
        );

    // text
    let text = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //center
                relative_1: Vec2{ x: 5., y: 15. },
                relative_2: Vec2{ x: 95., y: 97.5 },
                ..Default::default()
            }
        ).unwrap();

    let text_style = TextStyle {
            font      : asset_server.load(MISC_FONT),
            font_size : 45.0,
            color     : LOBBY_DISPLAY_FONT_COLOR,
        };

    let member_type = ListPage::member_type();
    let tag = match member_type
    {
        ClickLobbyMemberType::Player  => "Players",
        ClickLobbyMemberType::Watcher => "Watchers",
    };
    let default_text = match member_type
    {
        ClickLobbyMemberType::Player  => "Players: 00/00\0",  //want default text to have same width for each type
        ClickLobbyMemberType::Watcher => "Watchers: 00/00",
    };
    let text_entity = rcommands.commands().spawn(
            TextElementBundle::new(
                text,
                TextParams::center()
                    .with_style(&text_style)
                    .with_depth(100.)
                    .with_width(Some(90.)),
                default_text
            )
        ).id();

    // update the text when the lobby display changes
    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            {
                // define updated text
                let mut text_buffer = String::from(default_text);
                match world.resource::<ReactRes<LobbyDisplay>>().get()
                {
                    Some(lobby_contents) =>
                    {
                        let _ = write!(
                                text_buffer,
                                "{}: {}/{}",
                                tag,
                                lobby_contents.num(member_type),
                                lobby_contents.max(member_type),
                            );
                    }
                    None => ()
                };

                // update UI text
                syscall(world, (text_entity, text_buffer), update_ui_text);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_page_left_button<ListPage: ListPageTrait>(
    rcommands        : &mut ReactCommands,
    asset_server     : &AssetServer,
    ui               : &mut UiTree,
    area             : &Widget,
    content_entities : Arc<Vec<Entity>>,
){
    // button ui
    // - decrement page number and update display contents
    let button_overlay = make_overlay(ui, area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "<",
            move |world, _|
            {
                syscall(
                        world,
                        (
                            content_entities.clone(),
                            world.resource::<ReactRes<ListPage>>().get().saturating_sub(1),
                        ),
                        update_list_contents::<ListPage>
                    );
            }
        );

    // block button and grey-out text when the page number is zero
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<ListPage>(
            move | world: &mut World |
            {
                let enable = world.resource::<ReactRes<ListPage>>().get() != 0;
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_page_right_button<ListPage: ListPageTrait>(
    rcommands        : &mut ReactCommands,
    asset_server     : &AssetServer,
    ui               : &mut UiTree,
    area             : &Widget,
    content_entities : Arc<Vec<Entity>>,
){
    // button ui
    // - increment page number and update display contents
    let button_overlay = make_overlay(ui, area, "", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            ">",
            move |world, _|
            {
                syscall(
                        world,
                        (
                            content_entities.clone(),
                            world.resource::<ReactRes<ListPage>>().get().saturating_add(1),
                        ),
                        update_list_contents::<ListPage>
                    );
            }
        );

    // block button and grey-out text when the page number is maxed
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<ListPage>(
            move | world: &mut World |
            {
                let enable = !page_is_maxed::<ListPage>(
                        &world.resource::<ReactRes<LobbyDisplay>>(),
                        &world.resource::<ReactRes<ListPage>>()
                    );
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_contents<ListPage: ListPageTrait>(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // prepare content widgets
    let mut content_entities = Vec::with_capacity(NUM_LOBBY_CONTENT_ENTRIES);

    let entry_height = (1. / NUM_LOBBY_CONTENT_ENTRIES as f32) * 90.;

    for i in 0..NUM_LOBBY_CONTENT_ENTRIES
    {
        let text = Widget::create(
                ui,
                area.end(""),
                RelativeLayout{  //center
                    relative_1: Vec2{ x: 20., y: 5. + (i as f32)*entry_height },
                    relative_2: Vec2{ x: 80., y: 5. + ((i + 1) as f32)*entry_height },
                    ..Default::default()
                }
            ).unwrap();

        let text_style = TextStyle {
                font      : asset_server.load(MISC_FONT),
                font_size : 45.0,
                color     : LOBBY_DISPLAY_FONT_COLOR,
            };

        let text_entity = rcommands.commands().spawn(
                TextElementBundle::new(
                    text,
                    TextParams::centerleft()
                        .with_style(&text_style)
                        .with_width(Some(100.))
                        .with_height(Some(100.)),
                    "Watcher 00: ??????" //default value to set the scaling (want same scaling for all member types)
                )
            ).id();

        content_entities.push(text_entity);
    }

    let content_entities = Arc::new(content_entities);

    // update the contents when the lobby display changes
    let content_entities_clone = content_entities.clone();
    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            syscall(world, content_entities.clone(), update_display_list_contents_on_lobby_display::<ListPage>)
        );

    // paginate left button
    let page_left_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //bottom left corner
                relative_1: Vec2{ x: 1., y: 85. },
                relative_2: Vec2{ x: 15., y: 98. },
                ..Default::default()
            }
        ).unwrap();
    add_display_list_page_left_button::<ListPage>(
            rcommands,
            asset_server,
            ui,
            &page_left_area,
            content_entities_clone.clone(),
        );

    // paginate right button
    let page_right_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //bottom right corner
                relative_1: Vec2{ x: 85., y: 85. },
                relative_2: Vec2{ x: 99., y: 98. },
                ..Default::default()
            }
        ).unwrap();
    add_display_list_page_right_button::<ListPage>(
            rcommands,
            asset_server,
            ui,
            &page_right_area,
            content_entities_clone,
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_list<ListPage: ListPageTrait>(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    let list_overlay = make_overlay(ui, &area, "", true);

    // outline for entire area
    rcommands.commands().spawn(
            ImageElementBundle::new(
                    &list_overlay,
                    ImageParams::center()
                        .with_depth(50.1)
                        .with_width(Some(100.))
                        .with_height(Some(100.)),
                    asset_server.load(BOX),
                    Vec2::new(236.0, 139.0)
                )
        );

    // header
    let header_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 20. },
                ..Default::default()
            }
        ).unwrap();
    add_display_list_header::<ListPage>(rcommands, asset_server, ui, &header_area);

    // contents
    let contents_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 20. },
                relative_2: Vec2{ x: 100., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_display_list_contents::<ListPage>(rcommands, asset_server, ui, &contents_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_box(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // box for entire display
    //todo: it's better to place a box for the summary box area, but the box image gets too stretched
    let box_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 0. },
                relative_2: Vec2{ x: 90., y: 100. },
                ..Default::default()
            }
        ).unwrap();

    rcommands.commands().spawn(
            ImageElementBundle::new(
                    &box_area,
                    ImageParams::center()
                        .with_depth(50.)
                        .with_width(Some(100.))
                        .with_height(Some(100.)),
                    asset_server.load(BOX),
                    Vec2::new(236.0, 139.0)
                )
        );

    // summary box
    let summary_box_area = Widget::create(
            ui,
            box_area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 20. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_summary_box(rcommands, asset_server, ui, &summary_box_area);

    // players list
    let player_list_area = Widget::create(
            ui,
            box_area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0.5, y: 20. },
                relative_2: Vec2{ x: 50., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_list::<PlayerListPage>(rcommands, asset_server, ui, &player_list_area);

    // watchers list
    let watcher_list_area = Widget::create(
            ui,
            box_area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 50., y: 20. },
                relative_2: Vec2{ x: 99.5, y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_list::<WatcherListPage>(rcommands, asset_server, ui, &watcher_list_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_leave_lobby_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    button       : &Widget,
){
    // button ui
    let button_overlay = make_overlay(ui, button, "leave_lobby", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Leave",
            |world, _| syscall(world, (), leave_current_lobby)
        );

    // block button and grey-out text when there is no lobby
    let block_overlay = make_overlay(ui, &button_overlay, "", false);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            {
                let enable = world.resource::<ReactRes<LobbyDisplay>>().is_set();
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_start_game_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    button       : &Widget,
){
    // button ui
    let button_overlay = make_overlay(ui, button, "launch_lobby", true);
    let button_entity  = rcommands.commands().spawn_empty().id();

    make_basic_button(
            rcommands.commands(),
            asset_server,
            ui,
            &button_overlay,
            button_entity,
            "Start",
            |world, _| syscall(world, (), start_current_lobby)
        );

    // block button and grey-out text when we don't own the lobby
    let block_overlay = make_overlay(ui, &button_overlay, "", true);
    rcommands.commands().spawn((block_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World |
            {
                // only enable the button if we own the lobby
                let enable = match world.resource::<ReactRes<LobbyDisplay>>().get()
                {
                    Some(data) => data.owner_id == world.resource::<HostUserClient>().id(),
                    None       => false,
                };
                syscall(world, (enable, block_overlay.clone(), button_entity), toggle_button_availability);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_buttons(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // overlay
    let buttons_overlay = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 10. },
                relative_2: Vec2{ x: 90., y: 90. },
                ..Default::default()
            }
        ).unwrap();

    // buttons: widget grid
    let lobby_button_widgets = GridSegment::new()
        .with_cells(vec![
            GridCell::named(Vec2::splat(10.0), "leave_lobby"),
            GridCell::named(Vec2::splat(10.0), "start_game")
        ])
        .add_gaps(3.0)
        .build_in(ui, &buttons_overlay, GridOrientation::Horizontal)
        .unwrap();

    // prepare each of the buttons
    add_leave_lobby_button(rcommands, asset_server, ui, &lobby_button_widgets[0]);
    add_start_game_button(rcommands, asset_server, ui, &lobby_button_widgets[1]);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_display(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // title
    let lobby_display_title = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 15. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_title(rcommands, asset_server, ui, &lobby_display_title);

    // display box
    let lobby_display_box = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 15. },
                relative_2: Vec2{ x: 100., y: 75. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_box(rcommands, asset_server, ui, &lobby_display_box);

    // leave/launch buttons
    let lobby_leave_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 75. },
                relative_2: Vec2{ x: 100., y: 90. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_buttons(rcommands, asset_server, ui, &lobby_leave_button);

    // initialize UI listening to lobby display
    rcommands.trigger_resource_mutation::<LobbyDisplay>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyDisplayPlugin(app: &mut App)
{
    app.insert_resource(ReactRes::new(PlayerListPage(0)))
        .insert_resource(ReactRes::new(WatcherListPage(0)))
        ;
}

//-------------------------------------------------------------------------------------------------------------------
