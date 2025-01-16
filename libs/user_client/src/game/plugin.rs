use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::{ClientAppState, ClientFwConfig};
use bevy_girk_client_instance::ClientInstanceCommand;
use bevy_girk_game_fw::GameOverReport;
use client_core::ClientState;
use game::{handle_token_req, ClientInstanceReportPlugin, ClientStarterPlugin, LocalGamePlugin};
use game_core::ClickGameOverReport;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Resource will be set when a token request has failed. This allows the app to re-request connect tokens
/// without getting stuck in a 'can't do anything' state.
#[derive(Resource, Default)]
struct NeedTokenRequest(bool);

//-------------------------------------------------------------------------------------------------------------------

fn setup_game_tag_entities(mut c: Commands)
{
    let id = spawn_request_entity(&mut c, ConnectTokenRequest);
    c.entity(id)
        .on_event::<RequestFailed>()
        .r(|mut need_token: ResMut<NeedTokenRequest>| {
            need_token.0 = true;
        });
}

//-------------------------------------------------------------------------------------------------------------------

fn end_client_instance(mut c: Commands)
{
    c.queue(ClientInstanceCommand::End);
}

//-------------------------------------------------------------------------------------------------------------------

fn handle_need_token_request(
    mut c: Commands,
    mut need_token: ResMut<NeedTokenRequest>,
    config: Option<ClientFwConfig>,
    starter: ReactRes<ClientStarter>,
)
{
    if !need_token.0 {
        return;
    }
    need_token.0 = false;

    // don't request if in-game
    if config.is_some() {
        return;
    }

    // don't request if no cached starter
    let Some(game_id) = starter.game_id() else { return };

    c.syscall(game_id, handle_token_req);
}

//-------------------------------------------------------------------------------------------------------------------

// todo: maybe add 'waiting for game over report' screen to avoid it 'popping into view'
// game over -> waiting for report -> display report -> button to close report -> reports cached/available via
// API
fn log_game_over_report(event: BroadcastEvent<GameOverReport>)
{
    let report: ClickGameOverReport = event
        .read()
        .get()
        .expect("game over reports should deserialize");
    tracing::info!("{report:?}");
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug)]
pub(crate) struct ConnectTokenRequest;

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        let timer_configs = app.world().resource::<TimerConfigs>();
        let refresh = Duration::from_millis(timer_configs.token_request_loop_ms);

        app.add_plugins(ClientStarterPlugin)
            .add_plugins(ClientInstanceReportPlugin)
            .add_plugins(LocalGamePlugin)
            .add_systems(PreStartup, setup_game_tag_entities)
            .add_systems(OnEnter(ClientState::GameOver), end_client_instance)
            .add_reactor(broadcast::<GameOverReport>(), log_game_over_report)
            .add_systems(
                Last,
                handle_need_token_request
                    .run_if(in_state(ClientAppState::Client))
                    .run_if(on_timer(refresh)),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
