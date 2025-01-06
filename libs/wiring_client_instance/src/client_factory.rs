use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_client_instance::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_girk_wiring_common::*;
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
    type Data = ClientStarter;

    fn add_plugins(&mut self, app: &mut App)
    {
        // girk client config
        let config = GirkClientStartupConfig{
            resend_time: self.resend_time,
        };

        // set up client app
        prepare_girk_client_app(client_app, config);
        client_app
            .add_plugins(ClientCorePlugins)
            .add_plugins(ClickClientSkinPlugin);
    }

    fn setup_game(&mut self, world: &mut World, token: ServerConnectToken, start_info: ClientStartInfo<ClientStarter>)
    {
        let connect_pack = match ClientConnectPack::new(self.protocol_id, token) {
            Ok(connect) => connect,
            Err(err) => {
                tracing::error!("failed obtaining ClientConnectPack for {}: {err:?}", type_name::<Self>());
                return;
            }
        };

        // girk client config
        let config = GirkClientConfig{
            client_fw_config: start_info.data.client_fw_config,
            connect_pack,
        };

        // set up client app
        setup_girk_client_game(world, config);
        let client_id = start_info.client_id;
        let player_input_sender = prepare_client_core(&mut client_app, start_info.data.initializer);
        prepare_client_skin(&mut client_app, client_id, player_input_sender);

        match start_info.data.initializer
        {
            ClientInitializer::Player(player_initializer) =>
            {
                self.player_id    = Some(player_initializer.player_context.id());
                self.player_input = Some(setup_client_game_core(world, player_initializer));
            }
            ClientInitializer::Watcher => ()
        }

        // Enter the game.
        world.resource_mut::<NextState<ClientInstanceState>>().set(ClientInstanceState::Game);
    }
}

//-------------------------------------------------------------------------------------------------------------------
