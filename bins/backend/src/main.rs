use std::net::{IpAddr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_hub_server::*;
use bevy_girk_game_instance::*;
use bevy_girk_host_server::*;
use bevy_girk_utils::*;
use clap::Parser;
use enfync::AdoptOrDefault;
use game_core::*;
use renet2_setup::GameServerSetupConfig;
use wiring_backend::*;
use wiring_game_instance::*;

//-------------------------------------------------------------------------------------------------------------------

fn make_host_server_configs() -> HostServerStartupPack
{
    // configs
    let host_server_config = HostServerConfig {
        ticks_per_sec: Some(15),
        ongoing_game_purge_period_ticks: 1u64,
    };
    let lobbies_cache_config = LobbiesCacheConfig {
        max_request_size: LOBBY_LIST_SIZE as u16,
        lobby_checker: Box::new(ClickLobbyChecker {
            max_lobby_players: MAX_LOBBY_PLAYERS,
            max_lobby_watchers: MAX_LOBBY_WATCHERS,
            min_players_to_launch: MIN_PLAYERS_TO_LAUNCH,
        }),
    };
    let pending_lobbies_cache_config = PendingLobbiesConfig {
        ack_timeout: Duration::from_millis(ACK_TIMEOUT_MILLIS),
        start_buffer: Duration::from_secs(3),
    };
    let ongoing_games_cache_config = OngoingGamesCacheConfig { expiry_duration: Duration::from_secs(100) };
    let game_hub_disconnect_buffer_config =
        GameHubDisconnectBufferConfig { expiry_duration: Duration::from_secs(10) };

    HostServerStartupPack {
        host_server_config,
        lobbies_cache_config,
        pending_lobbies_cache_config,
        ongoing_games_cache_config,
        game_hub_disconnect_buffer_config,
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn make_hub_server_configs() -> GameHubServerStartupPack
{
    let game_hub_server_config = GameHubServerConfig {
        ticks_per_sec: Some(15),
        initial_max_capacity: 10u16,
        running_game_purge_period_ticks: 100u64,
    };
    let pending_games_cache_config = PendingGamesCacheConfig { expiry_duration: Duration::from_secs(2) };
    let running_games_cache_config = RunningGamesCacheConfig { expiry_duration: Duration::from_secs(100) };

    GameHubServerStartupPack {
        game_hub_server_config,
        pending_games_cache_config,
        running_games_cache_config,
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn make_click_game_configs(
    local_ip: Option<IpAddr>,
    proxy_ip: Option<IpAddr>,
    game_ticks_per_sec: u32,
    game_num_ticks: u32,
    ws_domain: Option<String>,
    wss_certs: Option<(PathBuf, PathBuf)>,
) -> ClickGameFactoryConfig
{
    // versioning
    //todo: use hasher directly?
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks = game_ticks_per_sec * 5;
    let game_prep_ticks = 0;
    let max_game_over_ticks = game_ticks_per_sec * 3;

    // server setup config
    let server_setup_config = GameServerSetupConfig {
        protocol_id,
        expire_secs: 10u64,
        timeout_secs: 5i32,
        server_ip: local_ip.unwrap_or(Ipv6Addr::LOCALHOST.into()),
        native_port: 0,
        wasm_wt_port: 0,
        wasm_ws_port: 0,
        proxy_ip,
        ws_domain,
        wss_certs,
        native_port_proxy: 0,
        wasm_ws_port_proxy: 0,
        wasm_wt_port_proxy: 0,
        has_wss_proxy: false,
    };

    // game framework config
    let game_fw_config = GameFwConfig::new(game_ticks_per_sec, max_init_ticks, max_game_over_ticks);

    // game duration config
    let duration_config = GameDurationConfig::new(game_prep_ticks, game_num_ticks);

    // click game factory config
    ClickGameFactoryConfig {
        server_setup_config,
        game_fw_config,
        duration_config,
        resend_time: Duration::from_millis(300),
    }
}

//-------------------------------------------------------------------------------------------------------------------

fn make_test_host_server(
    host_addr: String,
    rustls_config: Option<Arc<rustls::ServerConfig>>,
    configs: HostServerStartupPack,
) -> (App, url::Url, url::Url)
{
    // host-user server
    // - Make this first so host-hub server's wildcard port doesn't steal the server's pre-specified port.
    let acceptor = match rustls_config {
        Some(rustls_config) => bevy_simplenet::AcceptorConfig::Rustls(
            axum_server::tls_rustls::RustlsConfig::from_config(rustls_config),
        ),
        None => bevy_simplenet::AcceptorConfig::Default,
    };
    let host_user_server = host_user_server_factory().new_server(
        enfync::builtin::native::TokioHandle::adopt_or_default(),
        host_addr,
        acceptor,
        bevy_simplenet::Authenticator::None,
        bevy_simplenet::ServerConfig::default(),
    );
    let host_user_url = host_user_server.url();

    // host-hub server
    let host_hub_server = host_hub_server_factory().new_server(
        enfync::builtin::native::TokioHandle::adopt_or_default(),
        "127.0.0.1:0",
        bevy_simplenet::AcceptorConfig::Default,
        bevy_simplenet::Authenticator::None,
        bevy_simplenet::ServerConfig::default(),
    );
    let host_hub_url = host_hub_server.url();

    (
        make_host_server(configs, host_hub_server, host_user_server),
        host_hub_url,
        host_user_url,
    )
}

//-------------------------------------------------------------------------------------------------------------------

fn make_test_host_hub_client_with_id(client_id: u128, hub_server_url: url::Url) -> HostHubClient
{
    host_hub_client_factory().new_client(
        enfync::builtin::native::TokioHandle::adopt_or_default(),
        hub_server_url,
        bevy_simplenet::AuthRequest::None { client_id },
        bevy_simplenet::ClientConfig::default(),
        (),
    )
}

//-------------------------------------------------------------------------------------------------------------------

fn make_test_game_hub_server(
    game_instance_path: String,
    hub_server_url: url::Url,
    startup_pack: GameHubServerStartupPack,
    game_factory_config: ClickGameFactoryConfig,
) -> (Sender<GameHubCommand>, App)
{
    // setup
    let (command_sender, command_receiver) = new_channel::<GameHubCommand>();
    let host_hub_client = make_test_host_hub_client_with_id(0u128, hub_server_url);
    let game_launch_pack_source = GameLaunchPackSource::new(ClickGameLaunchPackSource::new(game_factory_config));
    let game_launcher = GameInstanceLauncher::new(GameInstanceLauncherProcess::new(
        game_instance_path,
        enfync::builtin::native::TokioHandle::adopt_or_default(),
    ));

    // server app
    let server_app = make_game_hub_server(
        startup_pack,
        command_receiver,
        host_hub_client,
        game_launch_pack_source,
        game_launcher,
    );

    (command_sender, server_app)
}

//-------------------------------------------------------------------------------------------------------------------

//todo: include log level
#[derive(Parser, Debug)]
struct BackendCli
{
    /// Specify the location of the game instance binary (will use the debug build directory by default).
    /// Requires '--game-instance'.
    #[arg(long)]
    game_instance: Option<String>,
    /// Address of user-host server.
    #[arg(long)]
    host_addr: Option<String>,
    /// Local IP for game servers.
    #[arg(long)]
    local_ip: Option<IpAddr>,
    /// Proxy IP for game servers.
    #[arg(long)]
    proxy_ip: Option<IpAddr>,
    /// Domain name for websocket game servers.
    #[arg(long)]
    ws_domain: Option<String>,
    /// File locations of tls certificates for websockets. See GameServerSetupConfig.
    ///
    /// Cert chain for websocket certs, should be `PEM` encoded.
    #[arg(long)]
    wss_certs: Option<String>,
    /// Privkey for websocket certs, should be `PEM` encoded.
    #[arg(long)]
    wss_certs_privkey: Option<String>,
}

//-------------------------------------------------------------------------------------------------------------------

const GAME_INSTANCE_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/game_instance");

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // logging
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env()
        .unwrap()
        .add_directive("bevy_simplenet=trace".parse().unwrap())
        .add_directive("renet2=info".parse().unwrap())
        .add_directive("renet2_netcode=info".parse().unwrap())
        .add_directive("renetcode2=info".parse().unwrap())
        .add_directive("bevy_girk_host_server=trace".parse().unwrap())
        .add_directive("bevy_girk_game_hub_server=trace".parse().unwrap())
        .add_directive("bevy_girk_wiring=trace".parse().unwrap())
        .add_directive("game_core=trace".parse().unwrap())
        .add_directive("bevy_girk_game_fw=trace".parse().unwrap());
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("launching backend: {:?}", std::env::args_os());

    // env
    let args = BackendCli::parse();
    tracing::info!(?args);

    // unwrap args
    let game_instance_path = args
        .game_instance
        .unwrap_or_else(|| String::from(GAME_INSTANCE_PATH));
    let host_addr = args.host_addr.unwrap_or_else(|| "127.0.0.1:48888".into());

    let wss_certs = match (args.wss_certs, args.wss_certs_privkey) {
        (Some(certs), Some(privkey)) => Some((PathBuf::from(certs), PathBuf::from(privkey))),
        (None, None) => None,
        (Some(_), None) => {
            tracing::error!("wss_certs arg found but wss_certs_privkey is missing");
            None
        }
        (None, Some(_)) => {
            tracing::error!("wss_certs_privkey arg found but wss_certs is missing");
            None
        }
    };
    let maybe_rustls = if let Some((certs, privkey)) = &wss_certs {
        GameServerSetupConfig::get_rustls_server_config(certs, privkey).ok()
    } else {
        None
    };

    // launch host server
    let (mut host_server, host_hub_url, host_user_url) =
        make_test_host_server(host_addr, maybe_rustls, make_host_server_configs());
    tracing::info!("host-user server running at {}", host_user_url.as_str());

    // launch game hub server attached to host server
    let game_ticks_per_sec = 20;
    let game_num_ticks = 20 * 30;

    // run the servers
    std::thread::spawn(move || {
        let (_hub_command_sender, mut hub_server) = make_test_game_hub_server(
            game_instance_path,
            host_hub_url,
            make_hub_server_configs(),
            make_click_game_configs(
                args.local_ip,
                args.proxy_ip,
                game_ticks_per_sec,
                game_num_ticks,
                args.ws_domain.clone(),
                wss_certs.clone(),
            ),
        );
        hub_server.run()
    });
    host_server.run();
}

//-------------------------------------------------------------------------------------------------------------------
