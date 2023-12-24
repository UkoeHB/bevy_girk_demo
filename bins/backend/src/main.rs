//module tree

//local shortcuts
use bevy_girk_demo_game_core::*;
use bevy_girk_demo_wiring_backend::*;
use bevy_girk_demo_wiring_game_instance::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_girk_game_fw::*;
use bevy_girk_game_hub_server::*;
use bevy_girk_game_instance::*;
use bevy_girk_host_server::*;
use bevy_girk_utils::*;
use bevy_kot_utils::*;

//standard shortcuts
use std::net::Ipv6Addr;
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_host_server_configs() -> HostServerStartupPack
{
    // configs
    let host_server_config = HostServerConfig{
            ticks_per_sec                   : Some(15),
            ongoing_game_purge_period_ticks : 1u64,
        };
    let lobbies_cache_config = LobbiesCacheConfig{
            max_request_size: LOBBY_LIST_SIZE as u16,
            lobby_checker: Box::new(ClickLobbyChecker{
                max_lobby_players     : MAX_LOBBY_PLAYERS as u16,
                max_lobby_watchers    : MAX_LOBBY_WATCHERS as u16,
                min_players_to_launch : MIN_PLAYERS_TO_LAUNCH as u16,
            })
        };
    let pending_lobbies_cache_config = PendingLobbiesConfig{
            ack_timeout  : Duration::from_millis(ACK_TIMEOUT_MILLIS),
            start_buffer : Duration::from_secs(3),
        };
    let ongoing_games_cache_config = OngoingGamesCacheConfig{
            expiry_duration: Duration::from_secs(100),
        };
    let game_hub_disconnect_buffer_config = GameHubDisconnectBufferConfig{
            expiry_duration: Duration::from_secs(10),
        };

    HostServerStartupPack{
            host_server_config,
            lobbies_cache_config,
            pending_lobbies_cache_config,
            ongoing_games_cache_config,
            game_hub_disconnect_buffer_config,
        }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_hub_server_configs() -> GameHubServerStartupPack
{
    let game_hub_server_config = GameHubServerConfig{
            ticks_per_sec                   : Some(15),
            initial_max_capacity            : 10u16,
            running_game_purge_period_ticks : 100u64,
        };
    let pending_games_cache_config = PendingGamesCacheConfig{
            expiry_duration: Duration::from_secs(2),
        };
    let running_games_cache_config = RunningGamesCacheConfig{
            expiry_duration: Duration::from_secs(100),
        };

    GameHubServerStartupPack{
            game_hub_server_config,
            pending_games_cache_config,
            running_games_cache_config,
        }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_click_game_configs(game_ticks_per_sec: Ticks, game_num_ticks: Ticks) -> ClickGameFactoryConfig
{
    // versioning
    //todo: use hasher directly
    let protocol_id = Rand64::new(env!("CARGO_PKG_VERSION"), 0u128).next();

    // config
    let max_init_ticks      = Ticks(game_ticks_per_sec.0 * 5);
    let game_prep_ticks     = Ticks(0);
    let max_game_over_ticks = Ticks(game_ticks_per_sec.0 * 3);

    // server setup config
    let server_setup_config = GameServerSetupConfig{
            protocol_id,
            expire_secs         : 10u64,
            server_timeout_secs : 5i32,
            client_timeout_secs : 15i32,
            server_ip           : Ipv6Addr::LOCALHOST,
        };

    // game framework config
    let game_fw_config = GameFWConfig::new(game_ticks_per_sec, max_init_ticks, max_game_over_ticks);

    // game duration config
    let game_duration_config = GameDurationConfig::new(game_prep_ticks, game_num_ticks);

    // click game factory config
    let game_factory_config = ClickGameFactoryConfig{
            server_setup_config,
            game_fw_config,
            game_duration_config,
        };

    game_factory_config
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_test_host_server(configs: HostServerStartupPack) -> (App, url::Url, url::Url)
{
    // host-hub server
    let host_hub_server = host_hub_server_factory().new_server(
            enfync::builtin::native::TokioHandle::default(),
            "127.0.0.1:0",
            bevy_simplenet::AcceptorConfig::Default,
            bevy_simplenet::Authenticator::None,
            bevy_simplenet::ServerConfig::default()
        );
    let host_hub_url = host_hub_server.url();

    // host-user server
    let host_user_server = host_user_server_factory().new_server(
            enfync::builtin::native::TokioHandle::default(),
            "127.0.0.1:48888",
            bevy_simplenet::AcceptorConfig::Default,
            bevy_simplenet::Authenticator::None,
            bevy_simplenet::ServerConfig::default()
        );
    let host_user_url = host_user_server.url();

    (
        make_host_server(configs, host_hub_server, host_user_server),
        host_hub_url,
        host_user_url,
    )
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_test_host_hub_client_with_id(client_id: u128, hub_server_url: url::Url) -> HostHubClient
{
    host_hub_client_factory().new_client(
            enfync::builtin::native::TokioHandle::default(),
            hub_server_url,
            bevy_simplenet::AuthRequest::None{ client_id },
            bevy_simplenet::ClientConfig::default(),
            ()
        )
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn make_test_game_hub_server(
    hub_server_url      : url::Url,
    startup_pack        : GameHubServerStartupPack,
    game_factory_config : ClickGameFactoryConfig,
) -> (Sender<GameHubCommand>, App)
{
    // setup
    let (command_sender, command_receiver) = new_channel::<GameHubCommand>();
    let host_hub_client         = make_test_host_hub_client_with_id(0u128, hub_server_url);
    let game_launch_pack_source = GameLaunchPackSource::new(ClickGameLaunchPackSource::new(&game_factory_config));
    let game_factory            = GameFactory::new(ClickGameFactory);
    let game_launcher           = GameInstanceLauncher::new(GameInstanceLauncherLocal::new(game_factory));

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
//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    // logging
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env().unwrap()
        .add_directive("bevy_simplenet=trace".parse().unwrap())
        .add_directive("renet=debug".parse().unwrap())
        .add_directive("renetcode=debug".parse().unwrap())
        .add_directive("bevy_girk_host_server=trace".parse().unwrap())
        .add_directive("bevy_girk_game_hub_server=trace".parse().unwrap())
        .add_directive("bevy_girk_wiring=trace".parse().unwrap())
        .add_directive("bevy_girk_demo_game_core=trace".parse().unwrap())
        .add_directive("bevy_girk_game_fw=trace".parse().unwrap());
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    // launch host server
    let (mut host_server, host_hub_url, _host_user_url) = make_test_host_server(make_host_server_configs());

    // launch game hub server attached to host server
    let game_ticks_per_sec = Ticks(20);
    let game_num_ticks     = Ticks(20 * 30);
    let (_hub_command_sender, mut hub_server) = make_test_game_hub_server(
            host_hub_url,
            make_hub_server_configs(),
            make_click_game_configs(game_ticks_per_sec, game_num_ticks)
        );

    // run the servers
    std::thread::spawn(move || hub_server.run());
    host_server.run();
}

//-------------------------------------------------------------------------------------------------------------------
