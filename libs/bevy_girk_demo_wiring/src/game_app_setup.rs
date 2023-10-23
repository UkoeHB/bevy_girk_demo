//local shortcuts
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Prepare a game app core.
///
/// Depends on game framework.
pub fn prepare_game_app_core(game_app: &mut App, game_initializer: ClickGameInitializer)
{
    game_app
        .add_plugins(GamePlugins)
        .insert_resource(game_initializer);
}

//-------------------------------------------------------------------------------------------------------------------
