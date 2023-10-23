//! Plugins for the core of a player client.
//!
//! PRECONDITION: plugin dependencies
//! - ClientCorePlugin
//!
//! PRECONDITION: the following must be initialized by the player client manager
//! - Res<ClickPlayerInitializer>
//! - Res<MessageReceiver<PlayerClientInput>>
//!

//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_girk_client_fw::*;
use bevy_girk_utils::*;
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

/// Validate resources that should exist before player client startup.
fn prestartup_check(world: &World)
{
    // check for expected resources
    if !world.contains_resource::<ClickPlayerInitializer>()
    { panic!("ClickPlayerInitializer is missing on startup!"); }
    if !world.contains_resource::<MessageReceiver<PlayerInput>>()
    { panic!("MessageReceiver<PlayerInput> is missing on startup!"); }

    check_client_framework_consistency(world.resource::<ClientFWConfig>(), world.resource::<ClickPlayerInitializer>());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Player client startup plugin.
#[bevy_plugin]
pub fn PlayerCoreStartupPlugin(app: &mut App)
{
    app.add_systems(PreStartup,
            (
                prestartup_check,
            )
        )
        .add_systems(Startup,
            (
                setup_player_state,
            )
        );
}

//-------------------------------------------------------------------------------------------------------------------

/// Player client tick plugin.
///
/// Configures system sets and adds basic administrative systems.
#[bevy_plugin]
pub fn PlayerCoreTickPlugin(app: &mut App)
{
    // Admin
    app.add_systems(Update,
                (
                    handle_player_client_inputs_init.in_set(ClientSet::InitStartup),
                    handle_player_client_inputs_prep.in_set(ClientSet::Prep),
                    handle_player_client_inputs_play.in_set(ClientSet::Play),
                    handle_player_client_inputs_gameover.in_set(ClientSet::GameOver),
                ).chain().in_set(ClientFWTickSet::Admin)
            );
}

//-------------------------------------------------------------------------------------------------------------------

pub struct PlayerCorePlugins;

impl PluginGroup for PlayerCorePlugins
{
    fn build(self) -> PluginGroupBuilder
    {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerCoreStartupPlugin)
            .add(PlayerCoreTickPlugin)
    }
}

//-------------------------------------------------------------------------------------------------------------------
