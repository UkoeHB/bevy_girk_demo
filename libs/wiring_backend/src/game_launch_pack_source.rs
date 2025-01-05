use std::collections::VecDeque;

use bevy_girk_backend_public::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_replicon::prelude::*;
#[cfg(not(target_family = "wasm"))]
use rand::seq::SliceRandom;
#[cfg(not(target_family = "wasm"))]
use rand::thread_rng;
use wiring_game_instance::*;

use crate::*;

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

fn make_player_init_data(
    env: bevy_simplenet::EnvType,
    user_id: u128,
    client_id: ClientId,
) -> ClickClientInitDataForGame
{
    let init = ClickClientInit::Player { client_id, player_name: format!("player{}", client_id.get()) };

    ClickClientInitDataForGame { env, user_id, init }
}

//-------------------------------------------------------------------------------------------------------------------

fn make_watcher_init_data(
    env: bevy_simplenet::EnvType,
    user_id: u128,
    client_id: ClientId,
) -> ClickClientInitDataForGame
{
    let init = ClickClientInit::Watcher { client_id };

    ClickClientInitDataForGame { env, user_id, init }
}

//-------------------------------------------------------------------------------------------------------------------

fn launch_pack_from_req(
    game_factory_config: &ClickGameFactoryConfig,
    start_request: &GameStartRequest,
) -> Result<GameLaunchPack, ()>
{
    // extract players/watchers from lobby data
    let Ok(lobby_contents) = ClickLobbyContents::try_from(start_request.lobby_data.clone()) else {
        tracing::error!("unable to extract lobby contents from lobby data");
        return Err(());
    };

    get_launch_pack(game_factory_config.clone(), lobby_contents)
}

//-------------------------------------------------------------------------------------------------------------------

pub fn get_launch_pack(
    game_factory_config: ClickGameFactoryConfig,
    mut lobby_contents: ClickLobbyContents,
) -> Result<GameLaunchPack, ()>
{
    // extract players/watchers from lobby contents
    let num_players = lobby_contents.players.len();
    let num_watchers = lobby_contents.watchers.len();

    // shuffle the game participants
    #[cfg(target_family = "wasm")]
    {
        if num_players != 1 {
            panic!("only single-player game instances are allowed on WASM");
        }
        if num_watchers != 0 {
            panic!("only single-player game instances are allowed on WASM");
        }
    }
    #[cfg(not(target_family = "wasm"))]
    {
        lobby_contents.players.shuffle(&mut thread_rng());
        lobby_contents.watchers.shuffle(&mut thread_rng());
    }

    // make init data for the clients
    let mut client_init_data = Vec::with_capacity(num_players + num_watchers);

    for (idx, (env, player_user_id)) in lobby_contents.players.iter().enumerate() {
        let client_id = ClientId::new(idx as u64);
        client_init_data.push(make_player_init_data(*env, *player_user_id, client_id));
    }

    for (idx, (env, watcher_user_id)) in lobby_contents.watchers.iter().enumerate() {
        let client_id = ClientId::new((idx + num_players) as u64);
        client_init_data.push(make_watcher_init_data(*env, *watcher_user_id, client_id));
    }

    // launch pack
    let data = ClickLaunchPackData { config: game_factory_config, clients: client_init_data };
    Ok(GameLaunchPack::new(lobby_contents.id, ser_msg(&data)))
}

//-------------------------------------------------------------------------------------------------------------------

pub struct ClickGameLaunchPackSource
{
    /// Serialized config needed by game factory to start a game.
    game_factory_config: ClickGameFactoryConfig,

    /// Queue of reports.
    queue: VecDeque<GameLaunchPackReport>,
}

impl ClickGameLaunchPackSource
{
    pub fn new(game_factory_config: ClickGameFactoryConfig) -> ClickGameLaunchPackSource
    {
        ClickGameLaunchPackSource { game_factory_config, queue: VecDeque::default() }
    }
}

impl GameLaunchPackSourceImpl for ClickGameLaunchPackSource
{
    /// Request a launch pack for a specified game.
    fn request_launch_pack(&mut self, start_request: &GameStartRequest)
    {
        match launch_pack_from_req(&self.game_factory_config, start_request) {
            Ok(launch_pack) => self
                .queue
                .push_back(GameLaunchPackReport::Pack(launch_pack)),
            Err(_) => self
                .queue
                .push_back(GameLaunchPackReport::Failure(start_request.game_id())),
        }
    }

    /// Get the next available report.
    fn try_next(&mut self) -> Option<GameLaunchPackReport>
    {
        self.queue.pop_front()
    }
}

//-------------------------------------------------------------------------------------------------------------------
