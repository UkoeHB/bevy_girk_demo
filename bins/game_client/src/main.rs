//module tree

//local shortcuts\
use bevy_girk_demo_wiring_client_instance::*;

//third-party shortcuts
use bevy_girk_client_instance::*;
use bevy_girk_utils::*;
use clap::Parser;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
struct GameClientCli
{
    #[clap(flatten)]
    instance: ClientInstanceCli,
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // log to stderr (not stdout, which is piped to the parent process for sending game instance reports)
    //todo: log to file instead (use CLI configs?)
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::TRACE.into())
        .from_env().unwrap()
        .add_directive("bevy=warn".parse().unwrap())
        .add_directive("bevy_app=warn".parse().unwrap())
        .add_directive("bevy_core=warn".parse().unwrap())
        .add_directive("bevy_winit=warn".parse().unwrap())
        .add_directive("bevy_render=warn".parse().unwrap())
        .add_directive("blocking=warn".parse().unwrap())
        .add_directive("naga_oil=warn".parse().unwrap())
        .add_directive("bevy_replicon=info".parse().unwrap());
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("game client started");

    // make client factory
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();
    let mut factory = ClientFactory::new(ClickClientFactory{ protocol_id });

    // run the client
    let args = GameClientCli::parse();
    inprocess_client_launcher(args.instance, &mut factory);
}

//-------------------------------------------------------------------------------------------------------------------
