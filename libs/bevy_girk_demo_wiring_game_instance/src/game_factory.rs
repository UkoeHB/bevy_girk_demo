//local shortcuts
use crate::*;
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_game_core::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_client_fw::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_girk_wiring::*;
use serde::{Deserialize, Serialize};

//standard shortcuts
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::vec::Vec;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

struct GameStartupHelper
{
    fw_init      : GameFwInitializer,
    click_init   : ClickGameInitializer,
    clients      : Vec<(u128, ClientIdType)>,
    native_count : usize,
    wasm_count   : usize,
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Prepare information to use when setting up the game app.
fn prepare_game_startup(
    client_init_data     : Vec<ClickClientInitDataForGame>,
    game_duration_config : GameDurationConfig
) -> Result<GameStartupHelper, ()>
{
    // prepare each client
    let mut client_states = Vec::<ClientState>::new();
    let mut players       = HashMap::<ClientIdType, PlayerState>::new();
    let mut watchers      = HashSet::<ClientIdType>::new();
    let mut clients       = Vec::<(u128, ClientIdType)>::new();
    let mut native_count  = 0;
    let mut wasm_count    = 0;
    client_states.reserve(client_init_data.len());
    players.reserve(client_init_data.len());
    watchers.reserve(client_init_data.len());
    clients.reserve(client_init_data.len());

    for client_init in client_init_data
    {
        // handle client type
        let client_id = match client_init.init
        {
            ClickClientInit::Player{ client_id, player_name } =>
            {
                players.insert(client_id, 
                        PlayerState{
                                id   : PlayerId { id: client_id },
                                name : PlayerName{ name: player_name },
                                ..default()
                            });
                client_id
            },
            ClickClientInit::Watcher{ client_id } =>
            {
                watchers.insert(client_id);
                client_id
            }
        };

        // make client state
        client_states.push(
                ClientState{
                        id            : ClientId::new(client_id),
                        access_rights :
                            InfoAccessRights{
                                    client : Some(client_id),
                                    global : true
                                }
                    }
            );

        // count client type
        match client_init.env
        {
            bevy_simplenet::EnvType::Native => native_count += 1,
            bevy_simplenet::EnvType::Wasm => wasm_count += 1,
        }

        // save user_id/client_id mapping
        clients.push((client_init.user_id, client_id));
    }

    // finalize
    let game_context = ClickGameContext::new(gen_rand128(), game_duration_config);

    Ok(GameStartupHelper{
        fw_init    : GameFwInitializer{ clients: client_states },
        click_init : ClickGameInitializer{ game_context, players, watchers },
        clients,
        native_count,
        wasm_count,
    })
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn client_initializer(
    game_initializer : &ClickGameInitializer,
    client_id        : ClientIdType,
) -> Result<ClientInitializer, ()>
{
    let client_type =
        'x : {
            if game_initializer.players.contains_key(&client_id) { break 'x ClientType::Player }
            if game_initializer.watchers.contains(&client_id)    { break 'x ClientType::Watcher }
            tracing::error!(client_id, "client is not a participant in the game");
            return Err(());
        };

    Ok(ClientInitializer{
            context: ClientContext::new(
                client_id,
                client_type,
                *game_initializer.game_context.duration_config(),
            )
        })
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn prepare_client_start_pack(
    game_initializer : &ClickGameInitializer,
    client_id        : ClientIdType,
    ticks_per_sec    : u32,
) -> Result<ClickClientStartPack, ()>
{
    // set up client framework
    let client_fw_config = ClientFwConfig::new(ticks_per_sec, client_id);

    // set up client config
    let client_initializer = client_initializer(game_initializer, client_id)?;

    Ok(ClickClientStartPack{ client_fw_config, client_initializer })
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Prepare game start report to assist participants with setting up their game clients.
fn get_game_start_infos(
    app          : &App,
    game_id      : u64,
    user_clients : &Vec<(u128, ClientIdType)>,
) -> Result<Vec<GameStartInfo>, ()>
{
    // extract data
    let game_initializer = app.world.resource::<ClickGameInitializer>();
    let ticks_per_sec    = app.world.resource::<GameFwConfig>().ticks_per_sec();

    // make start infos for each client
    let mut start_infos = Vec::<GameStartInfo>::new();
    start_infos.reserve(user_clients.len());

    for (user_id, client_id) in user_clients.iter()
    {
        // get game start package for client
        let client_start_pack = prepare_client_start_pack(&*game_initializer, *client_id, ticks_per_sec)?;

        // save client's start info
        start_infos.push(
                GameStartInfo{
                        game_id,
                        user_id               : *user_id,
                        client_id             : *client_id as u64,
                        serialized_start_data : ser_msg(&client_start_pack),
                    }
            );
    }

    Ok(start_infos)
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Configuration for setting up a game with a game factory.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClickGameFactoryConfig
{
    pub server_setup_config  : GameServerSetupConfig,
    pub game_fw_config       : GameFwConfig,
    pub game_duration_config : GameDurationConfig,
    pub resend_time          : Duration,
}

//-------------------------------------------------------------------------------------------------------------------

/// Client init data used when setting up a game.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClickClientInit
{
    Player{
        /// the client's in-game id
        client_id: ClientIdType,
        /// the client's player name
        player_name: String,
    },
    Watcher{
        /// the client's in-game id
        client_id: ClientIdType,
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Used by a game factory to initialize a client in the game.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClickClientInitDataForGame
{
    /// The client's environment type.
    pub env: bevy_simplenet::EnvType,

    /// The client's server-side user id.
    pub user_id: u128,

    /// Client init data for use in initializing a game.
    pub init: ClickClientInit,
}

//-------------------------------------------------------------------------------------------------------------------

/// Used by a game factory to initialize a game.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClickLaunchPackData
{
    /// Game config.
    pub config: ClickGameFactoryConfig,

    /// Client init data for use in initializing a game.
    pub clients: Vec<ClickClientInitDataForGame>,
}

//-------------------------------------------------------------------------------------------------------------------

/// Start-up pack for clients.
#[derive(Serialize, Deserialize)]
pub struct ClickClientStartPack
{
    /// Client framework config.
    pub client_fw_config: ClientFwConfig,
    /// Client initializer.
    pub client_initializer: ClientInitializer,
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ClickGameFactory;

impl GameFactoryImpl for ClickGameFactory
{
    fn new_game(&self, app: &mut App, launch_pack: GameLaunchPack) -> Result<GameStartReport, ()>
    {
        // extract game factory config
        let Some(data) = deser_msg::<ClickLaunchPackData>(&launch_pack.game_launch_data)
        else { tracing::error!("could not deserialize click game factory config"); return Err(()); };

        // initialize clients and game config
        let config = data.config;
        let startup = prepare_game_startup(data.clients, config.game_duration_config)?;

        // girk server config
        let server_config = GirkServerConfig{
            game_fw_config      : config.game_fw_config,
            game_fw_initializer : startup.fw_init,
            game_server_config  : config.server_setup_config,
            resend_time         : config.resend_time,
            native_count        : startup.native_count,
            wasm_count          : startup.wasm_count,
        };

        // prepare game app
        let (native_meta, wasm_meta) = prepare_girk_game_app(app, server_config);
        prepare_game_app_core(app, startup.click_init);

        // game start info
        // - must call this AFTER prepping the game app and setting up the renet server
        let start_infos = get_game_start_infos(&app, launch_pack.game_id, &startup.clients)?;

        Ok(GameStartReport{ native_meta, wasm_meta, start_infos })
    }
}

//-------------------------------------------------------------------------------------------------------------------
