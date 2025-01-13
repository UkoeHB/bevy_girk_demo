use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn try_reconnect(
    mut c: Commands,
    constructor: Res<HostClientConstructor>,
    mut status: ReactResMut<ConnectionStatus>,
    time: Res<Time>,
)
{
    if *status != ConnectionStatus::Dead {
        return;
    }

    tracing::info!("Constructing new host-user client...");
    c.insert_resource(constructor.new_client());
    *status.get_mut(&mut c) = ConnectionStatus::Connecting;
    c.react().broadcast(NewHostUserClient);
}

//-------------------------------------------------------------------------------------------------------------------

/// Stores a callback that produces [`HostUserClient`] on request.
///
/// Used to re-construct the client when it dies (which can happen, for example, if the server rejects
/// connections because it is over-capacity and we are using auth tokens that expire).
//todo: this workflow is not adequate for using auth tokens, which need to be obtained async from an http(s)
// server
#[derive(Resource)]
pub struct HostClientConstructor
{
    callback: Box<dyn Fn() -> HostUserClient + Send + Sync + 'static>,
}

impl HostClientConstructor
{
    pub fn new(callback: impl Fn() -> HostUserClient + Send + Sync + 'static) -> Self
    {
        Self { callback: Box::new(callback) }
    }

    pub fn new_client(&self) -> HostUserClient
    {
        (self.callback)()
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when a new host-user client is constructed.
pub(crate) struct NewHostUserClient;

//-------------------------------------------------------------------------------------------------------------------

#[derive(ReactResource, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ConnectionStatus
{
    Connecting,
    Connected,
    Dead,
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(SystemSet, Debug, Hash, PartialEq)]
pub(super) struct HostClientConnectSet;

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct HostClientConnectPlugin;

impl Plugin for HostClientConnectPlugin
{
    fn build(&self, app: &mut App)
    {
        let timer_configs = app.world.resource::<TimerConfigs>();
        let refresh = Duration::from_millis(timer_configs.host_reconstruct_loop_ms);

        app.insert_react_resource(ConnectionStatus::Dead)
            .add_systems(Startup, try_reconnect) // Make sure there is a client resource after startup.
            .add_systems(
                First,
                try_reconnect
                    .run_if(on_timer(refresh))
                    .in_set(HostClientConnectSet),
            );
    }
}

//-------------------------------------------------------------------------------------------------------------------
