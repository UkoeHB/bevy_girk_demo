//local shortcuts
use crate::*;

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
use std::time::Duration;

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
                relative_1: Vec2{ x: 0., y: 0. },
                relative_2: Vec2{ x: 98., y: 97. },
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
                    .with_height(Some(100.)),
                "Refreshing..."
            )
        );

    // setup reactors
    rcommands.commands().add(
            move |world: &mut World| syscall(world, overlay, setup_refresh_indicator_reactors)
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
    // prepare contents

    // update contents when lobby page changes
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

    // button: go to first page

    // button: paginate left

    // button: paginate right

    // button: go to last page
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

    //todo: block button when in-game
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
                relative_2: Vec2{ x: 100., y: 15. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_refresh_indicator(rcommands, asset_server, ui, &lobby_list_refresh_indicator);

    // list subsection
    let lobby_list_subsection = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 15. },
                relative_2: Vec2{ x: 100., y: 70. },
                ..Default::default()
            }
        ).unwrap();
    add_lobby_list_subsection(rcommands, asset_server, ui, &lobby_list_subsection);

    // navigation subsection
    let navigation_subsection = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 70. },
                relative_2: Vec2{ x: 100., y: 85. },
                ..Default::default()
            }
        ).unwrap();
    add_navigation_subsection(rcommands, asset_server, ui, &navigation_subsection);

    // new lobby button
    let new_lobby_button = Widget::create(
            ui,
            area.end(""),
            RelativeLayout{
                relative_1: Vec2{ x: 0., y: 85. },
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
        .add_systems(PostUpdate,
            (
                refresh_lobby_list, apply_deferred,
            ).chain()
                // refresh the list automatically if:
                // - in play section
                // - connected to host
                // - on timer OR just connected to host  (note: test timer first to avoid double-refresh when timer
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
