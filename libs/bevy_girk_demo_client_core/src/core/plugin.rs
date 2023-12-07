//! Plugins for the core of a player client.
//!
//! PRECONDITION: plugin dependencies
//! - bevy_replicon::core::ReplicationCorePlugin
//!
//! PRECONDITION: the following must be initialized by the client manager
//! - Res<ClientInitializer>
//! - Res<Receiver<PlayerInput>>
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

fn check_client_framework_consistency(client_fw_config: &ClientFWConfig, initializer: &ClientInitializer)
{
    // check the client id
    if client_fw_config.client_id() != initializer.context.id()
    { panic!("client id mismatch with client framework!"); }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Validate resources that should exist before client startup.
fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<ClientFWConfig>()
    { panic!("ClientFWConfig is missing on startup!"); }
    if !world.contains_resource::<ClientInitializer>()
    { panic!("ClientInitializer is missing on startup!"); }

    // validate consistency between client framework and core
    check_client_framework_consistency(world.resource::<ClientFWConfig>(), world.resource::<ClientInitializer>());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Client core sets.
///
/// These sets are modal in schedule `Update`.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ClientSet
{
    /// Runs the first time the client is initialized.
    InitStartup,
    /// Runs if the client is reinitialized after exiting 'Init' mode.
    InitReinit,
    /// Runs main initialization logic.
    InitCore,
    /// Runs in game mode 'prep' (but not when initializing).
    Prep,
    /// Runs in game mode 'play' (but not when initializing).
    Play,
    /// Runs in game mode 'game over' (but not when initializing).
    GameOver
}

//-------------------------------------------------------------------------------------------------------------------

/// Client startup plugin.
#[bevy_plugin]
pub fn ClientCoreStartupPlugin(app: &mut App)
{
    app.add_state::<ClientMode>()
        .add_systems(PreStartup,
            (
                prestartup_check,
            )
        )
        .add_systems(Startup,
            (
                setup_game_output_handler,
                setup_client_state,
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------

/// Client tick plugin.
///
/// Configures modal system sets and adds basic administrative systems.
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
    app.configure_sets(Update,
                ClientSet::InitStartup
                    .run_if(in_state(ClientFWMode::Init))
                    .run_if(in_state(ClientMode::Init))
            );

    // Init reinitialize. (runs if client needs to be reinitialized during a game)
    // - lock display and show reinitialization progress
    app.configure_sets(Update,
                ClientSet::InitReinit
                    .run_if(in_state(ClientFWMode::Init))  //framework is reinitializing
                    .run_if(not(in_state(ClientMode::Init)))  //client is not in init
            );

    // Init core. (always runs when framework is being initialized, regardless of client mode)
    // - connect to game and synchronize times
    app.configure_sets(Update,
                ClientSet::InitCore
                    .run_if(in_state(ClientFWMode::Init))
                    .after(ClientSet::InitStartup)
                    .after(ClientSet::InitReinit)
            );

    // Prep systems.
    app.configure_sets(Update,
                ClientSet::Prep
                    .run_if(in_state(ClientFWMode::Game))
                    .run_if(in_state(ClientMode::Prep))
            );

    // Play systems.
    app.configure_sets(Update,
                ClientSet::Play
                    .run_if(in_state(ClientFWMode::Game))
                    .run_if(in_state(ClientMode::Play))
            );

    // GameOver systems.
    app.configure_sets(Update,
                ClientSet::GameOver
                    .run_if(in_state(ClientFWMode::End))
                    .run_if(in_state(ClientMode::GameOver))
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
            // For this demo we assume watcher clients will re-use the player skin, which depends on `PlayerInputPlugin`.
            // A different project may want to completely separate player and watcher skins, in which case this plugin
            // can go in a player-client-specific crate.
            .add(PlayerInputPlugin)
    }
}

//-------------------------------------------------------------------------------------------------------------------
