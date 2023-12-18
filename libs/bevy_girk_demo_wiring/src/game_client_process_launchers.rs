//local shortcuts

//third-party shortcuts
use bevy_girk_game_instance::*;
use tokio::process::{Child, Command};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub fn launch_game_client(path: String, connect_info: &GameConnectInfo) -> Result<Child, ()>
{
    let connect_info_ser = serde_json::to_string(&connect_info).expect("failed serializing game connect info");

    Command::new(path.as_str())
        .args(["-G", connect_info_ser.as_str()])
        .spawn()
        .map_err(|_|())
}

//-------------------------------------------------------------------------------------------------------------------
