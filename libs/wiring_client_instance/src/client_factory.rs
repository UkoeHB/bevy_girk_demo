use std::any::type_name;
use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_client_instance::*;
use bevy_girk_wiring_client::{
    prepare_girk_client_app, setup_girk_client_game, GirkClientConfig, GirkClientStartupConfig,
};
use client_core::ClientCorePlugin;
use client_skin::ClientSkinPlugin;
use renet2_setup::{ClientConnectPack, ServerConnectToken};
use wiring_game_instance::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Client factory for standard click games.
///
/// Note: If the connection type is `InMemory`, then you must manually insert the in-memory client transport into
/// the       client app.
#[derive(Debug)]
pub struct ClickClientFactory
{
    pub protocol_id: u64,
    pub resend_time: Duration,
}

impl ClientFactoryImpl for ClickClientFactory
{
    type Data = ClientStartPack;

    /// Note: does not set up the user client, which is considered a semi-unrelated 'shell'
    fn add_plugins(&mut self, app: &mut App)
    {
        // girk client config
        let config = GirkClientStartupConfig { resend_time: self.resend_time };

        // set up client app
        prepare_girk_client_app(app, config);
        app.add_plugins(ClickClientGlobalPlugin)
            .add_plugins(ClientCorePlugin)
            .add_plugins(ClientSkinPlugin);
    }

    fn setup_game(
        &mut self,
        world: &mut World,
        token: ServerConnectToken,
        start_info: ClientStartInfo<ClientStartPack>,
    )
    {
        let connect_pack = match ClientConnectPack::new(self.protocol_id, token) {
            Ok(connect) => connect,
            Err(err) => {
                tracing::error!("failed obtaining ClientConnectPack for {}: {err:?}", type_name::<Self>());
                return;
            }
        };

        // girk client config
        let config = GirkClientConfig {
            client_fw_config: start_info.data.client_fw_config,
            connect_pack,
        };

        // set up client app
        setup_girk_client_game(world, config);
        setup_client_game(world, start_info.data.initializer);

        // We assume setup was triggered by a ClientInstanceCommand, which will set ClientAppState::Game.
    }
}

//-------------------------------------------------------------------------------------------------------------------
