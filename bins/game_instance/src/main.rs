//module tree

//local shortcuts
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_instance::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // make game factory
    let game_factory = GameFactory::new(ClickGameFactory);

    // launch the game
    // - the game launch pack will be extracted from environment args
    process_game_launcher(&mut std::env::args(), game_factory);
}

//-------------------------------------------------------------------------------------------------------------------
