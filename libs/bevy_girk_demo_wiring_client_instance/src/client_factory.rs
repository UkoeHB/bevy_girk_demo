use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_client_instance::*;
use bevy_girk_demo_wiring_game_instance::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_girk_wiring::*;

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
    fn new_client(&mut self, token: ServerConnectToken, start_info: GameStartInfo) -> Result<(App, u64), ()>
    {
        // new connect pack
        let connect_pack = RenetClientConnectPack::new(self.protocol_id, token).unwrap();

        // extract client startup pack
        let client_start_pack = deser_msg::<ClickClientStartPack>(&start_info.serialized_start_data).unwrap();

        // girk client config
        let config = GirkClientConfig {
            client_fw_config: client_start_pack.client_fw_config,
            resend_time: self.resend_time,
            connect_pack,
        };

        // set up client app
        let mut client_app = App::new();

        prepare_girk_client_app(&mut client_app, config);
        let client_id = client_start_pack.client_initializer.context.id();
        let player_input_sender = prepare_client_core(&mut client_app, client_start_pack.client_initializer);
        prepare_client_skin(&mut client_app, client_id, player_input_sender);

        Ok((client_app, self.protocol_id))
    }
}

//-------------------------------------------------------------------------------------------------------------------
