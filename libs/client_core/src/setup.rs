use bevy::prelude::*;
use bevy_girk_client_fw::*;
use game_core::*;
use wiring_game_instance::ClientInitializer;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn check_client_framework_consistency(client_fw_config: &ClientFwConfig, initializer: &ClientInitializer)
{
    // check the client id
    if client_fw_config.client_id() != initializer.context.id() {
        tracing::error!("client id mismatch with client framework on game startup!");
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Validate resources that should exist before client startup.
fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<ClientFwConfig>() {
        tracing::error!("ClientFwConfig is missing on game startup!");
    }
    if !world.contains_resource::<ClientInitializer>() {
        tracing::error!("ClientInitializer is missing on game startup!");
    }

    // validate consistency between client framework and core
    check_client_framework_consistency(
        world.resource::<ClientFwConfig>(),
        world.resource::<ClientInitializer>(),
    );
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
            .enable_state_scoped_entities::<ClientState>()
            .insert_resource(GameMessageHandler::new(handle_game_message))
            .insert_resource(ClientRequestType::new::<ClientRequest>())
            .add_systems(OnEnter(ClientAppState::Game), (prestartup_check, setup_client).chain());
    }
}

//-------------------------------------------------------------------------------------------------------------------
