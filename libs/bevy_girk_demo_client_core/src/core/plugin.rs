//! Plugins for the core of a player client.
//!
//! PRECONDITION: plugin dependencies
//! - bevy_replicon::core::ReplicationCorePlugin
//!

//local shortcuts
use crate::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_fn_plugin::*;
use bevy_girk_client_fw::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Validate resources that should exist before client startup.
fn prestartup_check(world: &World)
{
    // validate consistency between client framework and core
    if !world.contains_resource::<ClientFWConfig>()
    { panic!("ClientFWConfig is missing on startup!"); }
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
                setup_game_output_handler,
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------

/// Client tick plugin.
///
/// Configures system sets and adds basic administrative systems.
#[bevy_plugin]
pub fn ClientCoreTickPlugin(app: &mut App)
{
    // Admin
    /*
    app.add_systems(Update,
                (

                ).chain().in_set(ClientFWTickSet::Admin)
            );
    */

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

    // Misc
    // Systems that should run when the client is fully initialized.
    app.add_systems(OnEnter(ClientInitializationState::Done),
            (
                request_game_mode,
            ).chain()
        );
}

//-------------------------------------------------------------------------------------------------------------------

pub struct ClientCorePlugins;

impl PluginGroup for ClientCorePlugins
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
