//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_girk_wiring::*;
use bevy_kot_utils::*;
use bevy_renet::renet::transport::ClientAuthentication;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Prepare the core of a click game client.
///
/// Depends on client framework.
pub fn prepare_client_core(
    client_app  : &mut App,
    initializer : ClientInitializer
) -> Sender<PlayerInput>
{
    // player input channel
    let (player_input_sender, player_input_receiver) = new_channel::<PlayerInput>();

    // app
    client_app
        .add_plugins(ClientCorePlugins)
        .insert_resource(initializer)
        .insert_resource(player_input_receiver);

    player_input_sender
}

//-------------------------------------------------------------------------------------------------------------------

/// Make the core of a click game client.
///
/// Note: If the connection type is `InMemory`, then you must manually insert the in-memory client transport into the
///       client app.
pub fn make_game_client_core(
    expected_protocol_id : u64,
    connect_info         : GameConnectInfo
) -> (App, ClientIdType, Sender<PlayerInput>)
{
    // extract connect token and validate protocol version
    let ServerConnectToken::Native{ bytes: serialized_connect_token } = connect_info.server_connect_token;
    //else { panic!("only native connect tokens currently supported"); };

    let connect_token = connect_token_from_bytes(serialized_connect_token).unwrap();
    if connect_token.protocol_id != expected_protocol_id { panic!("protocol id mismatch"); }

    // extract client startup pack
    let client_start_pack = deser_msg::<ClickClientStartPack>(&connect_info.serialized_start_data).unwrap();

    // get client address based on server address
    let server_addr = connect_token.server_addresses[0].expect("only one server address is currently supported");
    let client_address = client_address_from_server_address(&server_addr);

    // set up client app
    let mut client_app = App::new();

    let client_fw_command_sender = prepare_client_app_framework(&mut client_app, client_start_pack.client_fw_config);
    prepare_client_app_replication(&mut client_app, client_fw_command_sender);
    prepare_client_app_network(
            &mut client_app,
            RenetClientConnectPack::Native(ClientAuthentication::Secure{ connect_token }, client_address),
        );
    let client_id = client_start_pack.client_initializer.context.id();
    let player_input_sender = prepare_client_core(&mut client_app, client_start_pack.client_initializer);

    (client_app, client_id, player_input_sender)
}

//-------------------------------------------------------------------------------------------------------------------
