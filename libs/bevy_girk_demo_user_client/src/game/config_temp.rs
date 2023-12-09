//local shortcuts
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use bevy_girk_utils::*;

//standard shortcuts
use std::net::Ipv6Addr;

//-------------------------------------------------------------------------------------------------------------------

//todo: inject these
pub(crate) const GAME_INSTANCE_PATH : &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_instance");
pub(crate) const GAME_CLIENT_PATH : &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_client");

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_click_game_configs() -> ClickGameFactoryConfig
{
    // game duration
    let game_ticks_per_sec = Ticks(20);
    let game_num_ticks = Ticks(game_ticks_per_sec.0 * 30);

    // versioning
    //todo: use hasher directly
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks  = Ticks(game_ticks_per_sec.0 * 5);
    let game_prep_ticks = Ticks(0);
    let game_over_ticks = Ticks(game_ticks_per_sec.0 * 3);

    // server setup config
    let server_setup_config = GameServerSetupConfig{
            protocol_id,
            expire_seconds  : 10u64,
            timeout_seconds : 5i32,
            server_ip       : Ipv6Addr::LOCALHOST,
        };

    // game framework config
    let game_fw_config = GameFWConfig::new(game_ticks_per_sec, max_init_ticks, game_over_ticks);

    // game duration config
    let game_duration_config = GameDurationConfig::new(game_prep_ticks, game_num_ticks);

    // click game factory config
    let game_factory_config = ClickGameFactoryConfig{
            server_setup_config,
            game_fw_config,
            game_duration_config,
        };

    game_factory_config
}

//-------------------------------------------------------------------------------------------------------------------
