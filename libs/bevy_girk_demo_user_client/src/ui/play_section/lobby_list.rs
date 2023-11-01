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
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    lobby_search  : Query<(Entity, &LobbyPageRequest), With<LobbySearch>>,
){
    tracing::trace!("refreshing lobby list");

    // re-request the last-requested lobby page
    let (entity, lobby_page_req) = lobby_search.single();
    rerequest_latest_lobby_page(&mut rcommands, &client, entity, &lobby_page_req);
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
    In(content_entities) : In<Arc<Vec<Entity>>>,
    mut text_query       : Query<&mut Text>,
    lobby_page           : Res<ReactRes<LobbyPage>>,
){
    // lobby list entries
    let entries = lobby_page.get();

    // update contents
    for (idx, content_entity) in content_entities.iter().enumerate()
    {
        // clear entry
        let Ok(mut text) = text_query.get_mut(*content_entity)
        else { tracing::error!("text entity is missing for lobby list contents"); return; };
        let text_section = &mut text.sections[0].value;
        text_section.clear();

        // get lobby list entry
        let Some(entry) = entries.get(idx) else { continue; };

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
            |world, _| syscall(world, (), refresh_lobby_list)
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

    // prepare content widgets
    let mut content_entities = Vec::with_capacity(LOBBY_LIST_SIZE as usize);

    let entry_height = (1. / LOBBY_LIST_SIZE as f32) * 95.;

    for i in 0..LOBBY_LIST_SIZE
    {
        let text = Widget::create(
                ui,
                area.end(""),
                RelativeLayout{  //center
                    relative_1: Vec2{ x: 10., y: 2.5 + (i as f32)*entry_height },
                    relative_2: Vec2{ x: 90., y: 2.5 + ((i + 1) as f32)*entry_height },
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
                    "Id: ??????, Owner: ??????, Players: 00/00, Watchers: 00/00"
                )
            ).id();

        content_entities.push(text_entity);
    }

    let content_entities = Arc::new(content_entities);

    // update contents when lobby page changes
    rcommands.add_resource_mutation_reactor::<LobbyPage>(
            move |world: &mut World| syscall(world, content_entities.clone(), update_lobby_list_contents)
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
            move | world: &mut World |
            {
                // define updated text
                let mut text_buffer = String::new();
                let _ = write!(
                        text_buffer,
                        "({}-{} / {})",
                        "????",  //todo: first index in page
                        "????",  //todo: last index in page
                        "????",  //todo: total lobbies in server
                    );

                // update UI text
                syscall(world, (text_entity, text_buffer), update_ui_text);
            }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_left_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_left_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_paginate_right_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn add_clamp_right_button(
    rcommands    : &mut ReactCommands,
    asset_server : &AssetServer,
    ui           : &mut UiTree,
    area         : &Widget,
){

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
                relative_1: Vec2{ x: 3., y: 5. },
                relative_2: Vec2{ x: 15., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_clamp_left_button(rcommands, asset_server, ui, &clamp_left_button_area);

    // button: paginate left
    let paginate_left_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //mid left
                relative_1: Vec2{ x: 17., y: 5. },
                relative_2: Vec2{ x: 29., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_paginate_left_button(rcommands, asset_server, ui, &paginate_left_button_area);

    // button: paginate right
    let paginate_right_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //mid right
                relative_1: Vec2{ x: 71., y: 5. },
                relative_2: Vec2{ x: 83., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_paginate_right_button(rcommands, asset_server, ui, &paginate_right_button_area);

    // button: go to last page
    let clamp_right_button_area = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{  //far right
                relative_1: Vec2{ x: 85., y: 5. },
                relative_2: Vec2{ x: 97., y: 95. },
                ..Default::default()
            }
        ).unwrap();
    add_clamp_right_button(rcommands, asset_server, ui, &clamp_right_button_area);
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
            |_world, _| () //syscall(world, (), activate_new_lobby_window)  //TODO
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
                relative_1: Vec2{ x: 0., y: 75. },
                relative_2: Vec2{ x: 100., y: 80. },
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
                // - on timer OR just connected to host  (note: test timer first to avoid double-refresh when timer
                //   is saturated)
                .run_if(|play_section: Query<(), (With<Selected>, With<MainPlayButton>)>| !play_section.is_empty() )
                .run_if(resource_equals(ConnectionStatus::Connected))
                //todo: not in game
                .run_if(on_timer(lobby_list_refresh)
                    .or_else(
                        |connection_status: Res<ConnectionStatus>| { connection_status.is_changed() }
                    )
                )
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
