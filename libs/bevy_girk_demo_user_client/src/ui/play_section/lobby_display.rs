//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;
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
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    mut lobby     : ReactResMut<LobbyDisplay>,
    leave_lobby   : Query<Entity, (With<LeaveLobby>, Without<React<PendingRequest>>)>,
){
    // check for existing request
    let Ok(target_entity) = leave_lobby.get_single()
    else { tracing::warn!("ignoring leave lobby request because a request is already pending"); return };

    // check if we are in a lobby
    let Some(lobby_id) = lobby.lobby_id()
    else { tracing::error!("tried to leave lobby but we aren't in a lobby"); return; };

    // leave the lobby
    match lobby.lobby_type()
    {
        None => { tracing::error!("tried to leave lobby but there is no lobby type"); return; }
        Some(LobbyType::Local) =>
        {
            // clear the lobby
            if lobby.is_set() { lobby.get_mut(&mut rcommands).clear(); }
        }
        Some(LobbyType::Hosted) =>
        {
            // send leave request
            let Ok(new_req) = client.request(UserToHostRequest::LeaveLobby{ id: lobby_id })
            else { tracing::warn!(lobby_id, "failed sending leave lobby request to host server"); return; };

            // save request
            rcommands.insert(target_entity, PendingRequest::new(new_req));
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn start_current_lobby(
    mut rcommands    : ReactCommands,
    client           : Res<HostUserClient>,
    mut lobby        : ReactResMut<LobbyDisplay>,
    mut game_monitor : ReactResMut<GameMonitor>,
    launch_lobby     : Query<Entity, (With<LaunchLobby>, Without<React<PendingRequest>>)>,
){
    // check for existing request
    let Ok(target_entity) = launch_lobby.get_single()
    else { tracing::warn!("ignoring start lobby request because a request is pending"); return };

    // check if we are in a lobby
    let Some(lobby_id) = lobby.lobby_id()
    else { tracing::error!("tried to start lobby but we aren't in a lobby"); return; };

    // check if there is an existing game
    if game_monitor.has_game() { tracing::error!("tried to start lobby but we are already in a game"); return; };

    // launch the lobby
    match lobby.lobby_type()
    {
        None => { tracing::error!("tried to start lobby but there is no lobby type"); return; }
        Some(LobbyType::Local) =>
        {
            // clear lobby display
            let Some(lobby_contents) = lobby.get_mut(&mut rcommands).clear()
            else { tracing::error!("lobby contents are missing in local lobby"); return; };

            // launch the game
            launch_local_player_game(game_monitor.get_mut(&mut rcommands), lobby_contents);
        }
        Some(LobbyType::Hosted) =>
        {
            // send launch reqeust
            let Ok(new_req) = client.request(UserToHostRequest::LaunchLobbyGame{ id: lobby_id })
            else { tracing::warn!(lobby_id, "failed sending launch lobby request to host server"); return; };

            // save request
            rcommands.insert(target_entity, PendingRequest::new(new_req));
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
                let _ = write!(text, "{} {}: {:0>6}", tag, member_number, member_id % 1_000_000u128);
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

fn add_lobby_display_title(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // set text style
    ui.add_style(basic_text_default_light_style());

    // spawn text
    let text = relative_widget(ui.tree(), area.end(""), (20., 80.), (40., 70.));
    spawn_basic_text(ui, text, TextParams::center().with_height(Some(100.)), "Current Lobby");
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_summary_box(ui: &mut UiBuilder<MainUi>, area: &Widget)
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
                    text.write(text_entity, 0, |text| write!(text, "Lobby: {:0>6} -- Owner: {:0>6}", id, owner_id)).unwrap();
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

fn add_display_list_header<ListPage: ListPageTrait>(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // outline for header
    spawn_plain_outline(ui, area.clone());

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

fn add_page_left_button<ListPage: ListPageTrait>(
    ui       : &mut UiBuilder<MainUi>,
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
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.rcommands.on(resource_mutation::<ListPage>(),
            move |mut ui: UiUtils<MainUi>, page: ReactRes<ListPage>|
            {
                let enable = page.get() != 0;
                ui.toggle_basic_button(enable, button_entity, &disable_overlay);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_page_right_button<ListPage: ListPageTrait>(
    ui       : &mut UiBuilder<MainUi>,
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
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.rcommands.on(resource_mutation::<ListPage>(),
            move |mut ui: UiUtils<MainUi>, display: ReactRes<LobbyDisplay>, page: ReactRes<ListPage>|
            {
                let enable = !page_is_maxed(&display, &*page);
                ui.toggle_basic_button(enable, button_entity, &disable_overlay);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_display_list_contents<ListPage: ListPageTrait>(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // prepare content widgets
    let mut contents = Vec::with_capacity(NUM_LOBBY_CONTENT_ENTRIES);

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

        contents.push(text_entity);
    }

    let contents = Arc::new(contents);

    // update the contents when the lobby display changes
    let contents_clone = contents.clone();
    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            prep_fncall(contents_clone.clone(), update_display_list_contents_on_lobby_display::<ListPage>)
        );

    // add page left/right buttons
    ui.div_rel(area.end(""), (1., 15.), (85., 98.), |ui, area| add_page_left_button::<ListPage>(ui, area, &contents));
    ui.div_rel(area.end(""), (85., 99.), (85., 98.), |ui, area| add_page_right_button::<ListPage>(ui, area, &contents));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_list<ListPage: ListPageTrait>(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // outline for entire area
    spawn_plain_outline(ui, area.clone());

    // fill the box
    ui.div_rel(area.end(""), (0., 100.), (0., 20.), add_display_list_header::<ListPage>);
    ui.div_rel(area.end(""), (0., 100.), (20., 100.), add_display_list_contents::<ListPage>);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_box(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // backdrop for entire display
    ui.div(|ui|{
        ui.add_style(ui.style::<LobbyDisplayStyle>().backdrop_box.clone());
        spawn_plain_box(ui, area.clone());
    });

    // outline above backdrop
    let outline = make_overlay(ui.tree(), &area, "", true);
    spawn_plain_outline(ui, outline.clone());

    // fill the box
    ui.div_rel(area.end(""), (0., 100.),  (0., 20.),   add_lobby_display_summary_box);
    ui.div_rel(area.end(""), (0.5, 50.),  (20., 100.), add_lobby_display_list::<PlayerListPage>);
    ui.div_rel(area.end(""), (50., 99.5), (20., 100.), add_lobby_display_list::<WatcherListPage>);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_leave_lobby_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    let default_text = "Leave";
    let button_entity = spawn_basic_button(ui, &area, default_text, leave_current_lobby);

    // disable button when there is no lobby
    let disable_overlay = spawn_basic_button_blocker(ui, &area, false);

    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move |mut ui: UiUtils<MainUi>, display: ReactRes<LobbyDisplay>|
            {
                let enable = display.is_set();
                ui.toggle_basic_button(enable, button_entity, &disable_overlay);
            }
        );

    // setup button reactors
    ui.commands().add(
            move |world: &mut World|
            {
                syscall(world, (button_entity, default_text), setup_simple_pending_button_text::<LeaveLobby>);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_start_game_button(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    // button ui
    let default_text = "Start";
    let button_entity = spawn_basic_button(ui, &area, default_text, start_current_lobby);

    // disable button
    // - when we don't own the lobby
    // - when the lobby is not ready to start
    // - when we are currently in a game
    let disable_overlay = make_overlay(ui.tree(), &area, "", true);
    ui.commands().spawn((disable_overlay.clone(), UiInteractionBarrier::<MainUi>::default()));

    ui.rcommands.on(resource_mutation::<LobbyDisplay>(),
            move
            |
                mut ui  : UiUtils<MainUi>,
                display : ReactRes<LobbyDisplay>,
                client  : Res<HostUserClient>,
                monitor : ReactRes<GameMonitor>
            |
            {
                let enable = match display.get()
                {
                    Some(data) =>
                    {
                        let owns = data.owner_id == client.id();
                        let single_player = display.is_local();
                        let can_launch_hosted = data.can_launch_hosted();

                        owns && (single_player || can_launch_hosted)
                    },
                    None => false,
                };
                let enable = enable && !monitor.has_game();
                ui.toggle_basic_button(enable, button_entity, &disable_overlay);
            }
        );

    // setup button reactors
    ui.commands().add(
            move |world: &mut World|
            {
                syscall(world, (button_entity, default_text), setup_simple_pending_button_text::<LaunchLobby>);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_buttons(ui: &mut UiBuilder<MainUi>, area: &Widget)
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

pub(crate) fn add_lobby_display(ui: &mut UiBuilder<MainUi>, area: &Widget)
{
    ui.div_rel(area.end(""), ( 0., 100.), ( 0., 15.), add_lobby_display_title);
    ui.div_rel(area.end(""), (10.,  90.), (15., 75.), add_lobby_display_box);
    ui.div_rel(area.end(""), ( 0., 100.), (75., 90.), add_lobby_buttons);

    // initialize UI
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
