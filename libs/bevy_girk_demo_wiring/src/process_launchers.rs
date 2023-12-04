//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_girk_game_instance::*;

//standard shortcuts
use std::process::{Child, Command};

//-------------------------------------------------------------------------------------------------------------------

pub fn launch_game_client(connect_info: GameConnectInfo) -> Child
{
    let connect_info_ser = serde_json::to_string(&connect_info).expect("failed serializing game connect info");

    Command::new("game_client")
        .args([GAME_CONNECT_INFO_TAG, connect_info_ser.as_str()])
        .spawn()
        .expect("failed launching game client")
}

//-------------------------------------------------------------------------------------------------------------------
