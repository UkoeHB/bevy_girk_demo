//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_girk_wiring::*;
use bevy_kot_utils::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Prepare the core of a click game client.
///
/// Depends on client framework.
pub fn prepare_client_core(
    client_app  : &mut App,
    initializer : ClientInitializer
) -> Sender<PlayerClientInput>
{
    // player input channel
    let (player_input_sender, player_input_receiver) = new_channel::<PlayerClientInput>();

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
    connect_token        : ServerConnectToken,
    start_info           : GameStartInfo
) -> (App, ClientIdType, Sender<PlayerClientInput>)
{
    // new connect pack
    let connect_pack = RenetClientConnectPack::new(expected_protocol_id, connect_token).unwrap();

    // extract client startup pack
    let client_start_pack = deser_msg::<ClickClientStartPack>(&start_info.serialized_start_data).unwrap();

    // set up client app
    let mut client_app = App::new();

    prepare_client_app_backend(&mut client_app, client_start_pack.client_fw_config, connect_pack);
    let client_id = client_start_pack.client_initializer.context.id();
    let player_input_sender = prepare_client_core(&mut client_app, client_start_pack.client_initializer);

    (client_app, client_id, player_input_sender)
}

//-------------------------------------------------------------------------------------------------------------------
