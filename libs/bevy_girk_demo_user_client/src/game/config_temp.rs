//local shortcuts
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_wiring_game_instance::*;

//third-party shortcuts
use bevy_girk_game_fw::*;
use bevy_girk_utils::*;

//standard shortcuts
use std::net::Ipv6Addr;
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn make_click_game_configs() -> ClickGameFactoryConfig
{
    // game duration
    let game_ticks_per_sec = 20;
    let game_num_ticks = game_ticks_per_sec * 30;

    // versioning
    //todo: use hasher directly
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks  = game_ticks_per_sec * 5;
    let game_prep_ticks = 0;
    let game_over_ticks = game_ticks_per_sec * 3;

    // server setup config
    let server_setup_config = GameServerSetupConfig{
            protocol_id,
            expire_secs  : 10u64,
            timeout_secs : 5i32,
            server_ip    : Ipv6Addr::LOCALHOST,
        };

    // game framework config
    let game_fw_config = GameFwConfig::new(game_ticks_per_sec, max_init_ticks, game_over_ticks);

    // game duration config
    let game_duration_config = GameDurationConfig::new(game_prep_ticks, game_num_ticks);

    // click game factory config
    let game_factory_config = ClickGameFactoryConfig{
            server_setup_config,
            game_fw_config,
            game_duration_config,
            resend_time: Duration::from_millis(300),
        };

    game_factory_config
}

//-------------------------------------------------------------------------------------------------------------------
