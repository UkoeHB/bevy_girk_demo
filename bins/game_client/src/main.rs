//module tree

//local shortcuts
use bevy_girk_demo_client_skin::*;
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_kot_utils::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn get_game_connect_info(mut args: std::env::Args) -> Result<GameConnectInfo, ()>
{
    // find connect info tag
    loop
    {
        match args.next()
        {
            Some(arg) => if arg == GAME_CONNECT_INFO_TAG { break; },
            None => return Err(()),
        }
    }

    // extract connect info
    let Some(arg) = args.next() else { return Err(()); };
    let connect_info = serde_json::de::from_str::<GameConnectInfo>(&arg).map_err(|_| ())?;

    Ok(connect_info)
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn prepare_game_client_skin(app: &mut App, _client_id: u64, player_input_sender: Sender<PlayerInput>)
{
    app.add_plugins(GIRKClientPluginTemp)
        .insert_resource(player_input_sender);
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // get connect info as input arg
    let connect_info = get_game_connect_info(std::env::args()).expect("game connect info is missing");

    // prep game core
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();
    let (mut app, client_id, player_input_sender) = make_game_client_core(protocol_id, connect_info);

    // prep game skin
    prepare_game_client_skin(&mut app, client_id as u64, player_input_sender);

    // run game to completion
    app.run();
}

//-------------------------------------------------------------------------------------------------------------------
