//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_instance::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn launch_local_player_game(monitor: &mut GameMonitor, lobby_contents: ClickLobbyContents)
{
    tracing::info!("launching local player game");

    let monitor_impl =
    {
        #[cfg(not(target_family = "wasm"))]
        {
            launch_local_player_game_native(lobby_contents)
        }

        #[cfg(target_family = "wasm")]
        {
            launch_local_player_game_wasm(lobby_contents)
        }
    };

    monitor.set(monitor_impl);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn launch_multiplayer_game(monitor: &mut GameMonitor, game_id: u64, connect_info: GameConnectInfo)
{
    let monitor_impl =
    {
        #[cfg(not(target_family = "wasm"))]
        {
            launch_multiplayer_game_native(game_id, connect_info)
        }

        #[cfg(target_family = "wasm")]
        {
            launch_multiplayer_game_wasm(game_id, lobby_contents)
        }
    };

    monitor.set(monitor_impl);
}

//-------------------------------------------------------------------------------------------------------------------
