//! Plugins for the core of a client.
//!
//! PRECONDITION: plugin dependencies
//! - bevy_replicon::core::ReplicationCorePlugin
//!
//! PRECONDITION: the following must be initialized by the client manager
//! - Res<ClickPlayerInitializer>
//! - Res<MessageReceiver<PlayerInput>>
//!

//local shortcuts
use crate::*;
use bevy_girk_client_fw::*;
use bevy_girk_demo_game_core::*;
use bevy_girk_utils::*;

//third-party shortcuts
use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_fn_plugin::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn check_client_framework_consistency(client_fw_config: &ClientFWConfig, player_initializer: &ClickPlayerInitializer)
{
    // check the client id
    if client_fw_config.client_id() != player_initializer.player_context.id()
        { panic!("client id mismatch with client framework!"); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Validate resources that should exist before client startup.
fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<ClickPlayerInitializer>()
        { panic!("ClickPlayerInitializer is missing on startup!"); }
    if !world.contains_resource::<MessageReceiver<PlayerInput>>()
        { panic!("MessageReceiver<PlayerInput> is missing on startup!"); }

    // validate consistency between client framework and core
    if !world.contains_resource::<ClientFWConfig>()
        { panic!("ClientFWConfig is missing on startup!"); }

    check_client_framework_consistency(world.resource::<ClientFWConfig>(), world.resource::<ClickPlayerInitializer>());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Client core sets.
/// These sets are modal.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ClientSet
{
    /// runs the first time the client is initialized
    InitStartup,
    /// runs if the client is reinitialized after exiting 'Init' mode
    InitReinit,
    /// runs main initialization logic
    InitCore,
    /// runs in game mode 'prep' (but not when initializing)
    Prep,
    /// runs in game mode 'play' (but not when initializing)
    Play,
    /// runs in game mode 'game over' (but not when initializing)
    GameOver
}

//-------------------------------------------------------------------------------------------------------------------

/// Client startup plugin.
#[bevy_plugin]
pub fn ClientCoreStartupPlugin(app: &mut App)
{
    app.add_state::<ClientCoreMode>()
        .add_systems(PreStartup,
            (
                prestartup_check,
            )
        )
        .add_systems(Startup,
            (
                setup_player_state,
                setup_game_output_handler,
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------

/// Client tick plugin.
/// Configures system sets and adds basic administrative systems.
#[bevy_plugin]
pub fn ClientCoreTickPlugin(app: &mut App)
{
    // Init startup. (runs on startup)
    // - load assets
    app.configure_set(Update,
                ClientSet::InitStartup
                    .run_if(in_state(ClientFWMode::Init))
                    .run_if(in_state(ClientCoreMode::Init))
            );

    // Init reinitialize. (runs if client needs to be reinitialized during a game)
    // - lock display and show reinitialization progress
    app.configure_set(Update,
                ClientSet::InitReinit
                    .run_if(in_state(ClientFWMode::Init))  //framework is reinitializing
                    .run_if(not(in_state(ClientCoreMode::Init)))  //client core is not in init
            );

    // Init core. (always runs when framework is being initialized, regardless of client mode)
    // - connect to game and synchronize times
    app.configure_set(Update,
                ClientSet::InitCore
                    .run_if(in_state(ClientFWMode::Init))
            );

    // Prep systems.
    app.configure_set(Update,
                ClientSet::Prep
                    .run_if(in_state(ClientFWMode::Game))
                    .run_if(in_state(ClientCoreMode::Prep))
            );

    // Play systems.
    app.configure_set(Update,
                ClientSet::Play
                    .run_if(in_state(ClientFWMode::Game))
                    .run_if(in_state(ClientCoreMode::Play))
            );

    // GameOver systems.
    app.configure_set(Update,
                ClientSet::GameOver
                    .run_if(in_state(ClientFWMode::End))
                    .run_if(in_state(ClientCoreMode::GameOver))
            );

    // Admin
    app.add_systems(Update,
                (
                    handle_player_client_inputs_init.in_set(ClientSet::InitStartup),
                    handle_player_client_inputs_prep.in_set(ClientSet::Prep),
                    handle_player_client_inputs_play.in_set(ClientSet::Play),
                    handle_player_client_inputs_gameover.in_set(ClientSet::GameOver),
                ).chain().in_set(ClientFWTickSet::Admin)
            );

    // Misc
    // Systems that should run when the client is fully initialized.
    app.add_systems(OnEnter(ClientInitializationState::Done),
            (
                request_game_mode,
            ).chain()
        );
}

//-------------------------------------------------------------------------------------------------------------------

pub struct ClientPlugins;

impl PluginGroup for ClientPlugins
{
    fn build(self) -> PluginGroupBuilder
    {
        PluginGroupBuilder::start::<Self>()
            .add(GameReplicationPlugin)
            .add(ClientCoreStartupPlugin)
            .add(ClientCoreTickPlugin)
    }
}

//-------------------------------------------------------------------------------------------------------------------
