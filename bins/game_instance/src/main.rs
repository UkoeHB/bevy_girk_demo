use bevy_girk_demo_wiring_game_instance::*;
use bevy_girk_game_instance::*;
use clap::Parser;

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // log to stderr (not stdout, which is piped to the parent process for sending game instance reports)
    //todo: log to file instead (use env::arg configs?)
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env()
        .unwrap();
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    // make game factory
    let game_factory = GameFactory::new(ClickGameFactory);

    // launch the game
    let args = GameInstanceCli::parse();
    inprocess_game_launcher(args, game_factory);
}

//-------------------------------------------------------------------------------------------------------------------
