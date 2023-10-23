//module tree

//local shortcuts

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // set asset directory location
    if let Err(err) = bevy_girk_utils::try_set_bevy_asset_root(2)
        { panic!("Could not set bevy asset root: {}", err.to_string()); }

    // build and launch the bevy app
    App::new()
        .add_plugins(bevy_girk_demo_user_client::ClickUserClientPlugin)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
