//local shortcuts

//third-party shortcuts
use bevy_girk_game_instance::*;
use tokio::process::{Child, Command};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Arg tag for passing game connect info into the game client subprocess.
pub const GAME_CONNECT_INFO_TAG: &'static str = "-gci";

//-------------------------------------------------------------------------------------------------------------------

pub fn launch_game_client(path: String, connect_info: &GameConnectInfo) -> Result<Child, ()>
{
    let connect_info_ser = serde_json::to_string(&connect_info).expect("failed serializing game connect info");

    Command::new(path.as_str())
        .args([GAME_CONNECT_INFO_TAG, connect_info_ser.as_str()])
        .spawn()
        .map_err(|_|())
}

//-------------------------------------------------------------------------------------------------------------------
