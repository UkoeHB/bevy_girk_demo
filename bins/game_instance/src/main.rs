//module tree

//local shortcuts
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_girk_game_instance::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // log to stderr (not stdout, which is piped to the parent process for sending game instance reports)
    //todo: log to file instead (use env::arg configs?)
    // /*
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::WARN)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // */

    // make game factory
    let game_factory = GameFactory::new(ClickGameFactory);

    // launch the game
    // - the game launch pack will be extracted from environment args
    process_game_launcher(&mut std::env::args(), game_factory);
}

//-------------------------------------------------------------------------------------------------------------------
