use bevy::prelude::*;
use bevy_girk_client_fw::*;
use game_core::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn check_client_framework_consistency(client_fw_config: &ClientFwConfig, initializer: &ClientInitializer)
{
    // check the client id
    if client_fw_config.client_id() != initializer.context.id() {
        panic!("client id mismatch with client framework!");
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Validate resources that should exist before client startup.
fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<ClientFwConfig>() {
        panic!("ClientFwConfig is missing on startup!");
    }
    if !world.contains_resource::<ClientInitializer>() {
        panic!("ClientInitializer is missing on startup!");
    }

    // validate consistency between client framework and core
    check_client_framework_consistency(
        world.resource::<ClientFwConfig>(),
        world.resource::<ClientInitializer>(),
    );
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the client fw.
pub(crate) fn setup_fw_reqs(mut commands: Commands)
{
    commands.insert_resource(GameMessageHandler::new(handle_game_message));
    commands.insert_resource(ClientRequestType::new::<ClientRequest>());
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the client on game start.
pub(crate) fn setup_client(world: &mut World)
{
    let initializer = world
        .remove_resource::<ClientInitializer>()
        .expect("initializer missing");
    world.insert_resource(initializer.context);
}

//-------------------------------------------------------------------------------------------------------------------

/// Client startup plugin.
pub(crate) struct ClientSetupPlugin;

impl Plugin for ClientSetupPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_sub_state::<ClientState>()
            .add_systems(PreStartup, prestartup_check)
            .add_systems(Startup, setup_fw_reqs)
            .add_systems(OnEnter(ClientAppState::Game), setup_client);
    }
}

//-------------------------------------------------------------------------------------------------------------------
