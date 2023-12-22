//local shortcuts
use crate::*;
use bevy_girk_demo_wiring_game_instance::*;

//third-party shortcuts
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;

//standard shortcuts
#[cfg(not(target_family = "wasm"))]
use rand::thread_rng;
#[cfg(not(target_family = "wasm"))]
use rand::seq::SliceRandom;
use std::collections::VecDeque;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

// Use this method in the crate that instantiates a launch pack source.
/*
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_protocol_id() -> u64
{
    let mut hasher = AHasher::default();
    PACKAGE_VERSION.hash(&mut hasher);
    hasher.finish()
}
*/

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_player_init_data(env: bevy_simplenet::EnvType, user_id: u128, client_id: ClientIdType) -> ClientInitDataForGame
{
    let client_init_data = ClickClientInitDataForGame::Player{
            client_id   : client_id,
            player_name : format!("player{}", client_id),
        };

    ClientInitDataForGame{
            env,
            user_id,
            data: ser_msg(&client_init_data),
        }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_watcher_init_data(env: bevy_simplenet::EnvType, user_id: u128, client_id: ClientIdType) -> ClientInitDataForGame
{
    let client_init_data = ClickClientInitDataForGame::Watcher{
            client_id: client_id,
        };

    ClientInitDataForGame{
            env,
            user_id,
            data: ser_msg(&client_init_data),
        }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn launch_pack_from_req(game_factory_config_ser: &Vec<u8>, start_request: &GameStartRequest) -> Result<GameLaunchPack, ()>
{
    // extract players/watchers from lobby data
    let Ok(lobby_contents) = ClickLobbyContents::try_from(start_request.lobby_data.clone())
    else { tracing::error!("unable to extract lobby contents from lobby data"); return Err(()); };

    get_launch_pack(game_factory_config_ser.clone(), lobby_contents)
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub fn get_launch_pack(
    game_factory_config_ser : Vec<u8>,
    mut lobby_contents      : ClickLobbyContents,
) -> Result<GameLaunchPack, ()>
{
    // extract players/watchers from lobby contents
    let num_players  = lobby_contents.players.len();
    let num_watchers = lobby_contents.watchers.len();

    // shuffle the game participants
    #[cfg(target_family = "wasm")]
    {
        if num_players != 1 { panic!("only single-player game instances are allowed on WASM"); }
        if num_watchers != 0 { panic!("only single-player game instances are allowed on WASM"); }
    }
    #[cfg(not(target_family = "wasm"))]
    {
        lobby_contents.players.shuffle(&mut thread_rng());
        lobby_contents.watchers.shuffle(&mut thread_rng());
    }

    // make init data for the clients
    let mut client_init_data = Vec::<ClientInitDataForGame>::new();
    client_init_data.reserve(num_players + num_watchers);

    for (idx, (env, player_user_id)) in lobby_contents.players.iter().enumerate()
    {
        client_init_data.push(make_player_init_data(*env, *player_user_id, idx as ClientIdType));
    }

    for (idx, (env, watcher_user_id)) in lobby_contents.watchers.iter().enumerate()
    {
        let client_id = idx + num_players;
        client_init_data.push(make_watcher_init_data(*env, *watcher_user_id, client_id as ClientIdType));
    }

    // launch pack
    Ok(GameLaunchPack::new(lobby_contents.id, game_factory_config_ser, client_init_data))
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct ClickGameLaunchPackSource
{
    /// Serialized config needed by game factory to start a game.
    game_factory_config_ser: Vec<u8>,

    /// Queue of reports.
    queue: VecDeque<GameLaunchPackReport>,
}

impl ClickGameLaunchPackSource
{
    pub fn new(game_factory_config: &ClickGameFactoryConfig) -> ClickGameLaunchPackSource
    {
        ClickGameLaunchPackSource{ game_factory_config_ser: ser_msg(&game_factory_config), queue: VecDeque::default() }
    }
}

impl GameLaunchPackSourceImpl for ClickGameLaunchPackSource
{
    /// Request a launch pack for a specified game.
    fn request_launch_pack(&mut self, start_request: &GameStartRequest)
    {
        match launch_pack_from_req(&self.game_factory_config_ser, start_request)
        {
            Ok(launch_pack) => self.queue.push_back(GameLaunchPackReport::Pack(launch_pack)),
            Err(_)          => self.queue.push_back(GameLaunchPackReport::Failure(start_request.game_id())),
        }
    }

    /// Get the next available report.
    fn try_next(&mut self) -> Option<GameLaunchPackReport>
    {
        self.queue.pop_front()
    }
}

//-------------------------------------------------------------------------------------------------------------------
