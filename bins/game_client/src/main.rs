//module tree

//local shortcuts
use bevy_girk_demo_client_core::*;
use bevy_girk_demo_client_skin::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_instance::*;
use bevy_girk_utils::*;
use bevy_kot_utils::*;
use clap::Parser;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
struct GameClientCli
{
    #[arg(short = 'T', value_parser = parse_json::<ServerConnectToken>)]
    token: ServerConnectToken,
    #[arg(short = 'G', value_parser = parse_json::<GameStartInfo>)]
    start_info: GameStartInfo,
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn prepare_game_client_skin(app: &mut App, _client_id: u64, player_input_sender: Sender<PlayerClientInput>)
{
    app.add_plugins(ClickClientSkinPlugin)
        .insert_resource(player_input_sender);
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

    // CLI args
    let args = GameClientCli::parse();

    // prep game core
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();
    let (
            mut app,
            client_id,
            player_input_sender
        ) = make_game_client_core(protocol_id, args.token, args.start_info);

    // prep game skin
    prepare_game_client_skin(&mut app, client_id as u64, player_input_sender);

    // run game to completion
    app.run();
}

//-------------------------------------------------------------------------------------------------------------------
