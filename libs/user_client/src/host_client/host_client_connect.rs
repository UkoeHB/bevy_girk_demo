use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Default, Deref, DerefMut)]
struct NextConstructTime(Duration);

//-------------------------------------------------------------------------------------------------------------------

fn try_reconnect(
    mut c: Commands,
    mut next_construct_time: ResMut<NextConstructTime>,
    constructor: Res<HostClientConstructor>,
    mut status: ReactResMut<ConnectionStatus>,
    time: Res<Time>,
)
{
    if *status != ConnectionStatus::Dead {
        return;
    }

    if timer.elapsed() < *next_construct_time {
        return;
    }

    *next_construct_time = timer.elapsed() + Duration::from_millis(500);
    c.insert_resource(constructor.new_client());
    *status.get_mut(&mut c) = ConnectionStatus::Connecting;
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

#[derive(SystemSet, Debug, Hash, PartialEq)]
pub(super) struct HostClientConnectSet;

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct HostClientConnectPlugin;

impl Plugin for HostClientConnectPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_react_resource(ConnectionStatus::Dead)
            .init_resource::<NextConstructTime>()
            .add_systems(Startup, try_reconnect)
            .add_systems(First, try_reconnect.in_set(HostClientConnectSet));
    }
}

//-------------------------------------------------------------------------------------------------------------------
