//local shortcuts
use crate::*;
use bevy_girk_game_fw::*;

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts
use std::collections::HashMap;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_misc_resources(world: &mut World, game_context: &ClickGameContext)
{
    world.insert_resource::<GameRand>(GameRand::new(game_context.seed()));
    world.insert_resource::<GameTick>(GameTick::default());
    world.insert_resource::<PrepTick>(PrepTick::default());
    world.insert_resource::<PlayTick>(PlayTick::default());
    world.insert_resource::<GameOverTick>(GameOverTick::default());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Initializes the state of clients.
pub(crate) fn setup_game_state(world: &mut World)
{
    // extract initializer
    let initializer = world.remove_resource::<ClickGameInitializer>().expect("initializer missing");

    // misc resources
    setup_misc_resources(world, &initializer.game_context);

    // game context
    world.insert_resource(initializer.game_context);

    // players
    // - player map
    // - player entities
    let mut client_entity_map = HashMap::<ClientId, Entity>::default();

    for (_, player_state) in initializer.players
    {
        // [ client id : entity ]
        let mut entity_commands = world.spawn_empty();
        client_entity_map.insert(player_state.id.id, entity_commands.id());

        // add player entity
        entity_commands.insert(player_state);
    }

    world.insert_resource(PlayerMap::new(client_entity_map));

    // watchers
    // - watcher map
    world.insert_resource(WatcherMap::new(initializer.watchers));
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the game message buffer.
pub(crate) fn setup_game_message_buffer(mut commands: Commands)
{
    commands.insert_resource(GameMessageType::new::<GameMsg>());
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the game input handler.
pub(crate) fn setup_game_input_handler(mut commands: Commands)
{
    commands.insert_resource(ClientRequestHandler::new(handle_client_request));
}

//-------------------------------------------------------------------------------------------------------------------
