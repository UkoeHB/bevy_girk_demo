//local shortcuts

//third-party shortcuts
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use tokio::process::{Child, Command};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub fn launch_game_client(path: String, token: &ServerConnectToken, start_info: &GameStartInfo) -> Result<Child, ()>
{
    let token_ser = serde_json::to_string(token).expect("failed serializing game connect token");
    let start_ser = serde_json::to_string(start_info).expect("failed serializing game start info");

    Command::new(path.as_str())
        .args(["-T", token_ser.as_str()])
        .args(["-S", start_ser.as_str()])
        .spawn()
        .map_err(|_|())
}

//-------------------------------------------------------------------------------------------------------------------
