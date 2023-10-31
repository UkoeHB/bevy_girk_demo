//local shortcuts
use crate::*;

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

/// Tracks the current player list page. Starts at 0.
#[derive(Resource, Debug)]
struct PlayerListPage(usize);

/// Tracks the current watcher list page. Starts at 0.
#[derive(Resource, Debug)]
struct WatcherListPage(usize);

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

fn update_player_list_contents(
    In((
        content_entities,
        new_page,
    ))                   : In<(Arc<Vec<Entity>>, usize)>,
    mut rcommands        : ReactCommands,
    mut text_query       : Query<&mut Text>,
    lobby                : Res<ReactRes<LobbyDisplay>>,
    mut player_list_page : ResMut<ReactRes<PlayerListPage>>,
){
    // get list index for first element
    let first_idx = new_page * NUM_LOBBY_CONTENT_ENTRIES;

    // update contents
    for (idx, content_entity) in content_entities.iter().enumerate()
    {
        let player_idx    = first_idx + idx;
        let player_number = first_idx + idx + 1;

        // clear entry
        let Ok(mut text) = text_query.get_mut(*content_entity)
        else { tracing::error!("text entity is missing for player list contents"); return; };
        let text_section = &mut text.sections[0].value;
        text_section.clear();

        // check if the entry corresponds to a player slot
        let Some(lobby_contents) = lobby.get() else { continue; };
        if player_number > lobby_contents.config.max_players as usize { continue; }

        // update entry text
        match lobby_contents.players.get(player_idx)
        {
            None =>
            {
                let _ = write!(text_section, "Player {}:", player_number);
            }
            Some(lobby_player) =>
            {
                let player_id: u128 = lobby_player.1;
                let _ = write!(text_section, "Player {}: {}", player_number, player_id % 1_000_000u128);
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
            if first_idx < lobby_contents.config.max_players as usize
            {
                new_page
            }
            else
            {
                (lobby_contents.config.max_players as usize / NUM_LOBBY_CONTENT_ENTRIES)
                    .saturating_sub(
                            1usize.saturating_sub(
                                    lobby_contents.config.max_players as usize % NUM_LOBBY_CONTENT_ENTRIES
                                )
                        )
            }
        }
    };
    player_list_page.get_mut(&mut rcommands).0 = new_page;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Update player list contents when the lobby display is mutated.
fn update_player_list_contents_on_lobby_display(
    In(content_entities) : In<Arc<Vec<Entity>>>,
    mut last_lobby_id    : Local<Option<u64>>,
    mut commands         : Commands,
    lobby                : Res<ReactRes<LobbyDisplay>>,
    player_list_page     : Res<ReactRes<PlayerListPage>>,
){
    // get next list page
    // - we reset to 0 when the lobby changes
    // - only change the page if the lobby id has changed
    let current_lobby_id = lobby.lobby_id();

    let next_list_page = match *last_lobby_id == current_lobby_id
    {
        true  => player_list_page.0,
        false => 0,
    };

    // update the recorded lobby id
    *last_lobby_id = current_lobby_id;

    // update the player list contents
    // note: it would be more efficient to use world access directly here, but you can't have Locals in exclusive systems
    commands.add(
           move |world: &mut World| syscall(world, (content_entities, next_list_page), update_player_list_contents)
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

fn add_player_list_header(
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
                    Vec2::new(237.0, 140.0)
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

    let default_text = "Players: 00/00";
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
                                "Players: {}/{}",
                                lobby_contents.players.len(),
                                lobby_contents.config.max_players
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

fn add_player_list_page_left_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_player_list_page_right_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_player_list_contents(
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
                        .with_width(Some(100.)),
                    "Player 00: ??????"
                )
            ).id();

        content_entities.push(text_entity);
    }

    let content_entities = Arc::new(content_entities);

    // update the contents when the lobby display changes
    rcommands.add_resource_mutation_reactor::<LobbyDisplay>(
            move | world: &mut World | syscall(world, content_entities.clone(), update_player_list_contents_on_lobby_display)
        );

    // paginate left button
    let page_left_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 80. },
                relative_2: Vec2{ x: 20., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_player_list_page_left_button(rcommands, asset_server, ui, &page_left_area);

    // paginate right button
    let page_right_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 80., y: 80. },
                relative_2: Vec2{ x: 100., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_player_list_page_right_button(rcommands, asset_server, ui, &page_right_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_lobby_display_player_list(
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
                    asset_server.load(OUTLINE),
                    Vec2::new(237.0, 140.0)
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
    add_player_list_header(rcommands, asset_server, ui, &header_area);

    // contents
    let contents_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 20. },
                relative_2: Vec2{ x: 100., y: 80. },
                ..Default::default()
            }
        ).unwrap();
    add_player_list_contents(rcommands, asset_server, ui, &contents_area);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_watcher_list_header(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){
    // add text
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

    let default_text = "Watchers: 00/00";
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
                                "Watchers: {}/{}",
                                lobby_contents.watchers.len(),
                                lobby_contents.config.max_watchers
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

fn add_lobby_display_watcher_list(
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
                    asset_server.load(OUTLINE),
                    Vec2::new(237.0, 140.0)
                )
        );

    // outline for header
    let header_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 100., y: 20. },
                ..Default::default()
            }
        ).unwrap();

    rcommands.commands().spawn(
            ImageElementBundle::new(
                    &header_area,
                    ImageParams::center()
                        .with_depth(50.1)
                        .with_width(Some(100.))
                        .with_height(Some(100.)),
                    asset_server.load(OUTLINE),
                    Vec2::new(237.0, 140.0)
                )
        );
    add_watcher_list_header(rcommands, asset_server, ui, &header_area);

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
                    Vec2::new(237.0, 140.0)
                )
        );

    // summary box
    let summary_box_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 0. },
                relative_2: Vec2{ x: 90., y: 20. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_summary_box(rcommands, asset_server, ui, &summary_box_area);

    // players list
    let player_list_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 10., y: 20. },
                relative_2: Vec2{ x: 50., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_player_list(rcommands, asset_server, ui, &player_list_area);

    // watchers list
    let watcher_list_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 50., y: 20. },
                relative_2: Vec2{ x: 90., y: 100. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_display_watcher_list(rcommands, asset_server, ui, &watcher_list_area);
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
