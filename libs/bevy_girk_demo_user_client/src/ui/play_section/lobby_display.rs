//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts
use std::fmt::Write;
use std::sync::Arc;
use std::vec::Vec;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

trait ListPageTrait: ReactResource + Send + Sync + 'static
{
    fn set(&mut self, page: usize);
    fn get(&self) -> usize;
    fn member_type() -> ClickLobbyMemberType;
}

/// Tracks the current player list page. Starts at 0.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
struct PlayerListPage(usize);

impl ListPageTrait for PlayerListPage
{
    fn set(&mut self, page: usize) { self.0 = page; }
    fn get(&self) -> usize { self.0 }
    fn member_type() -> ClickLobbyMemberType { ClickLobbyMemberType::Player }
}

/// Tracks the current watcher list page. Starts at 0.
///
/// This is a reactive resource.
#[derive(ReactResource, Debug)]
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
    client     : Res<HostUserClient>,
    lobby      : ReactRes<LobbyDisplay>,
    //join_lobby : Query<Entity, (With<JoinLobby>, Without<React<PendingRequest>>)>,
){
    //todo: need request/response pattern for LeaveLobby
    // check for existing request
    //if !join_lobby.is_empty()
    //{ tracing::warn!("ignoring leave lobby request because a request is already pending"); return }

    // check if we are in a lobby
    let Some(lobby_id) = lobby.lobby_id()
    else { tracing::error!("tried to leave lobby but we aren't in a lobby"); return; };

    // leave the lobby
    match lobby.lobby_type()
    {
        None => { tracing::error!("tried to leave lobby but there is no lobby type"); return; }
        Some(LobbyType::Local) =>
        {
            //todo: clear lobby contents
            tracing::error!("leaving local lobby not yet supported");
        }
        Some(LobbyType::Hosted) =>
        {
            if let Err(err) = client.send(UserToHostMsg::LeaveLobby{ id: lobby_id })
            { tracing::warn!(?err, lobby_id, "failed sending leave lobby message to host server"); }
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn start_current_lobby(
    client : Res<HostUserClient>,
    lobby  : ReactRes<LobbyDisplay>,
){
    let Some(lobby_id) = lobby.lobby_id()
    else { tracing::error!("tried to start lobby but we aren't in a lobby"); return; };

    match lobby.lobby_type()
    {
        None => { tracing::error!("tried to start lobby but there is no lobby type"); return; }
        Some(LobbyType::Local) =>
        {
            //todo: clear lobby contents
            //todo: start game
            tracing::error!("starting local lobby not yet supported");
        }
        Some(LobbyType::Hosted) =>
        {
            if let Err(err) = client.send(UserToHostMsg::LaunchLobbyGame{ id: lobby_id })
            { tracing::warn!(?err, lobby_id, "failed sending launch lobby message to host server"); }

            //todo: request/response pattern? and make the start lobby button do nothing when waiting
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn update_list_contents<ListPage: ListPageTrait>(
    In((
        content_entities,
        new_page,
    ))                   : In<(Arc<Vec<Entity>>, usize)>,
    mut rcommands        : ReactCommands,
    mut text_handle      : TextHandle,
    lobby                : ReactRes<LobbyDisplay>,
    mut member_list_page : ReactResMut<ListPage>,
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
        let Ok(text) = text_handle.text(*content_entity, 0)
        else { tracing::error!("text entity is missing for list contents"); return; };
        text.clear();

        // check if the entry corresponds to a member slot
        let Some(lobby_contents) = lobby.get() else { continue; };
        if member_number > lobby_contents.max(member_type) as usize { continue; }

        // update entry text
        match lobby_contents.get_id(member_type, member_idx)
        {
            None =>
            {
                let _ = write!(text, "{} {}:", tag, member_number);
            }
            Some(member_id) =>
            {
                let _ = write!(text, "{} {}: {}", tag, member_number, member_id % 1_000_000u128);
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
    lobby                : ReactRes<LobbyDisplay>,
    member_list_page     : ReactRes<ListPage>,
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

fn add_lobby_display_title(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // set text style
    ui.add(basic_text_default_light_style());

    // spawn text
    let text = relative_widget(ui.tree(), area.end(""), (20., 80.), (40., 70.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(100.)), "Current Lobby");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_summary_box(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // add text to summary box (center)
    let text = relative_widget(ui.tree(), area.end(""), (5., 95.), (15., 97.5));

    let default_text = "Lobby: ?????? -- Owner: ??????";
    let text_entity = spawn_basic_text(ui, text, TextParams::center().with_width(Some(90.)), default_text);

    // update the text when the lobby display changes
    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move |mut text: TextHandle, display: ReactRes<LobbyDisplay>|
            {
                if let Some(lobby_contents) = display.get()
                {
                    let id       = lobby_contents.id % 1_000_000u64;
                    let owner_id = lobby_contents.owner_id % 1_000_000u128;
                    text.write(text_entity, 0, |text| write!(text, "Lobby: {} -- Owner: {}", id, owner_id)).unwrap();
                }
                else
                {
                    text.write(text_entity, 0, |text| write!(text, "{}", default_text)).unwrap();
                }
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_header<ListPage: ListPageTrait>(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // outline for header
    spawn_plain_outline(ui, area.clone(), None);

    // text
    let text = relative_widget(ui.tree(), area.end(""), (5., 95.), (15., 97.5));

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
    let text_entity = spawn_basic_text(ui, text, TextParams::center().with_width(Some(90.)), default_text);

    // update the text when the lobby display changes
    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move |mut text: TextHandle, display: ReactRes<LobbyDisplay>|
            {
                if let Some(lobby_contents) = display.get()
                {
                    let num_members = lobby_contents.num(member_type);
                    let max_members = lobby_contents.max(member_type);
                    text.write(text_entity, 0, |text| write!(text, "{}: {}/{}", tag, num_members, max_members)).unwrap();
                }
                else
                {
                    text.write(text_entity, 0, |text| write!(text, "{}", default_text)).unwrap();
                }
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_page_left_button<ListPage: ListPageTrait>(
    ui       : &mut UiBuilder<MainUI>,
    area     : &Widget,
    entities : &Arc<Vec<Entity>>,
){
    // button ui
    // - decrement page number and update display contents
    let content_entities = entities.clone();
    let button_entity = spawn_basic_button(ui, &area, "<",
            move |world: &mut World|
            {
                syscall(
                        world,
                        (
                            content_entities.clone(),
                            world.react_resource::<ListPage>().get().saturating_sub(1),
                        ),
                        update_list_contents::<ListPage>
                    );
            }
        );

    // disable button when the page number is zero
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<ListPage>(),
            move |mut ui: UiUtils<MainUI>, page: ReactRes<ListPage>|
            {
                let enable = page.get() != 0;
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_page_right_button<ListPage: ListPageTrait>(
    ui       : &mut UiBuilder<MainUI>,
    area     : &Widget,
    entities : &Arc<Vec<Entity>>,
){
    // button ui
    // - increment page number and update display contents
    let content_entities = entities.clone();
    let button_entity = spawn_basic_button(ui, &area, ">",
            move |world: &mut World|
            {
                syscall(
                        world,
                        (
                            content_entities.clone(),
                            world.react_resource::<ListPage>().get().saturating_add(1),
                        ),
                        update_list_contents::<ListPage>
                    );
            }
        );

    // disable button when the page number is maxed
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<ListPage>(),
            move |mut ui: UiUtils<MainUI>, display: ReactRes<LobbyDisplay>, page: ReactRes<ListPage>|
            {
                let enable = !page_is_maxed(&display, &*page);
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_contents<ListPage: ListPageTrait>(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // prepare content widgets
    let mut content_entities = Vec::with_capacity(NUM_LOBBY_CONTENT_ENTRIES);

    let entry_height = (1. / NUM_LOBBY_CONTENT_ENTRIES as f32) * 90.;

    for i in 0..NUM_LOBBY_CONTENT_ENTRIES
    {
        // text in center
        let y_range = (
                5. + (i       as f32)*entry_height,
                5. + ((i + 1) as f32)*entry_height
            );
        let text = relative_widget(ui.tree(), area.end(""), (20., 80.), y_range);

        let text_entity = spawn_basic_text(
                ui,
                text,
                TextParams::centerleft()
                    .with_width(Some(100.)),
                "Watcher 00: ??????" //default value to set the scaling (want same scaling for all member types)
            );

        content_entities.push(text_entity);
    }

    let content_entities = Arc::new(content_entities);

    // update the contents when the lobby display changes
    let content_entities_clone = content_entities.clone();
    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            syscall_with(content_entities_clone.clone(), update_display_list_contents_on_lobby_display::<ListPage>)
        );

    // paginate left button (bottom left corner)
    let page_left_area = relative_widget(ui.tree(), area.end(""), (1., 15.), (85., 98.));
    ui.div(|ui| add_display_list_page_left_button::<ListPage>(ui, &page_left_area, &content_entities));

    // paginate right button (bottom right corner)
    let page_right_area = relative_widget(ui.tree(), area.end(""), (85., 99.), (85., 98.));
    ui.div(|ui| add_display_list_page_right_button::<ListPage>(ui, &page_right_area, &content_entities));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_list<ListPage: ListPageTrait>(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // box for entire area
    spawn_plain_box(ui, area.clone(), None);

    // header
    let header_area = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 20.));
    ui.div(|ui| add_display_list_header::<ListPage>(ui, &header_area));

    // contents
    let contents_area = relative_widget(ui.tree(), area.end(""), (0., 100.), (20., 100.));
    ui.div(|ui| add_display_list_contents::<ListPage>(ui, &contents_area));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_box(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // box for entire display
    //todo: it's better to place a box for the summary box area, but the box image gets too stretched
    let box_area = relative_widget(ui.tree(), area.end(""), (10., 90.), (0., 100.));
    spawn_plain_box(ui, box_area.clone(), None);

    // summary box
    let summary_box_area = relative_widget(ui.tree(), box_area.end(""), (0., 100.), (0., 20.));
    ui.div(|ui| add_lobby_display_summary_box(ui, &summary_box_area));

    // players list
    let player_list_area = relative_widget(ui.tree(), box_area.end(""), (0.5, 50.), (20., 100.));
    ui.div(|ui| add_lobby_display_list::<PlayerListPage>(ui, &player_list_area));

    // watchers list
    let watcher_list_area = relative_widget(ui.tree(), box_area.end(""), (50., 99.5), (20., 100.));
    ui.div(|ui| add_lobby_display_list::<WatcherListPage>(ui, &watcher_list_area));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_leave_lobby_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    let button_entity = spawn_basic_button(ui, &area, "Leave", leave_current_lobby);

    // disable button when there is no lobby
    let disable_overlay = make_overlay(ui.tree(), &area, "", false);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move |mut ui: UiUtils<MainUI>, display: ReactRes<LobbyDisplay>|
            {
                let enable = display.is_set();
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_start_game_button(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // button ui
    let button_entity = spawn_basic_button(ui, &area, "Start", start_current_lobby);

    // disable button when we don't own the lobby
    let disable_overlay = make_overlay(ui.tree(), &area, "", true);
    ui.commands().spawn((disable_overlay.clone(), UIInteractionBarrier::<MainUI>::default()));

    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move |mut ui: UiUtils<MainUI>, display: ReactRes<LobbyDisplay>, client: Res<HostUserClient>|
            {
                let enable = match display.get()
                {
                    Some(data) => data.owner_id == client.id(),
                    None       => false,
                };
                ui.toggle_basic_button(enable, &disable_overlay, button_entity);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_buttons(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // overlay
    let buttons_overlay = relative_widget(ui.tree(), area.end(""), (10., 90.), (10., 90.));

    // buttons: widget grid
    let lobby_button_widgets = GridSegment::new()
        .with_cells(vec![
            GridCell::named(Vec2::splat(10.0), "leave_lobby"),
            GridCell::named(Vec2::splat(10.0), "start_game")
        ])
        .add_gaps(3.0)
        .build_in(ui.tree(), &buttons_overlay, GridOrientation::Horizontal)
        .unwrap();

    // prepare each of the buttons
    ui.div(|ui| add_leave_lobby_button(ui, &lobby_button_widgets[0]));
    ui.div(|ui| add_start_game_button(ui, &lobby_button_widgets[1]));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_lobby_display(ui: &mut UiBuilder<MainUI>, area: &Widget)
{
    // title
    let lobby_display_title = relative_widget(ui.tree(), area.end(""), (0., 100.), (0., 15.));
    ui.div(|ui| add_lobby_display_title(ui, &lobby_display_title));

    // display box
    let lobby_display_box = relative_widget(ui.tree(), area.end(""), (0., 100.), (15., 75.));
    ui.div(|ui| add_lobby_display_box(ui, &lobby_display_box));

    // leave/launch buttons
    let lobby_leave_button = relative_widget(ui.tree(), area.end(""), (0., 100.), (75., 90.));
    ui.div(|ui| add_lobby_buttons(ui, &lobby_leave_button));

    // initialize UI listening to lobby display
    ui.rcommands.trigger_resource_mutation::<LobbyDisplay>();
    ui.rcommands.trigger_resource_mutation::<PlayerListPage>();
    ui.rcommands.trigger_resource_mutation::<WatcherListPage>();
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiLobbyDisplayPlugin(app: &mut App)
{
    app.insert_react_resource(PlayerListPage(0))
        .insert_react_resource(WatcherListPage(0))
        ;
}

//-------------------------------------------------------------------------------------------------------------------
