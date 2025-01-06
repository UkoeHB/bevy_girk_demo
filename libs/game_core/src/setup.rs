use std::collections::HashMap;

use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_replicon::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Initializes the game framework.
fn setup_fw_reqs(mut commands: Commands)
{
    commands.insert_resource(GameMessageType::new::<GameMsg>());
    commands.insert_resource(ClientRequestHandler::new(handle_client_request));
}

//-------------------------------------------------------------------------------------------------------------------

/// Sets up the game with injected initialization data.
fn setup_game(world: &mut World)
{
    // extract initializer
    let initializer = world
        .remove_resource::<ClickGameInitializer>()
        .expect("ClickGameInitializer missing on startup");

    // resources
    world.insert_resource::<GameRand>(GameRand::new(initializer.game_context.seed()));
    world.insert_resource(initializer.game_context);

    // players
    // - player map
    // - player entities
    let mut client_entity_map = HashMap::<ClientId, Entity>::default();

    for (_, player_state) in initializer.players {
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

pub(crate) struct GameSetupPlugin;

impl Plugin for GameSetupPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, (setup_fw_reqs, setup_game));
    }
}

//-------------------------------------------------------------------------------------------------------------------
