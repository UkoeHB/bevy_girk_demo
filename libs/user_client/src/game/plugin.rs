use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_user_client_utils::*;
use bevy_girk_utils::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn setup_game_tag_entities(mut c: Commands)
{
    c.spawn(ConnectTokenRequest);
}

//-------------------------------------------------------------------------------------------------------------------

fn end_client_instance(mut c: Commands)
{
    c.queue(ClientInstanceCommand::End);
}

//-------------------------------------------------------------------------------------------------------------------

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
        app.add_plugins(ClientStarterPlugin)
            .add_plugins(ClientInstanceReportPlugin)
            .add_plugins(LocalGamePlugin)
            .add_systems(PreStartup, setup_game_tag_entities)
            .add_systems(OnEnter(ClientState::GameOver), end_client_instance)
            .add_reactor(broadcast::<GameOverReport>(), log_game_over_report);
    }
}

//-------------------------------------------------------------------------------------------------------------------
