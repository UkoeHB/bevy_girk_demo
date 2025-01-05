use bevy::prelude::*;
use bevy_kot_utils::*;
use bevy_replicon::prelude::*;
use client_core::*;
use client_skin::*;

//-------------------------------------------------------------------------------------------------------------------

/// Prepare the core of a click game client.
pub fn prepare_client_core(client_app: &mut App, initializer: ClientInitializer) -> Sender<PlayerClientInput>
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

/// Prepare the click client skin on a client app.
pub fn prepare_client_skin(app: &mut App, _client_id: ClientId, player_input_sender: Sender<PlayerClientInput>)
{
    app.add_plugins(ClickClientSkinPlugin)
        .insert_resource(player_input_sender);
}

//-------------------------------------------------------------------------------------------------------------------
