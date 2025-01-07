use bevy::prelude::*;
use bevy_cobweb::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct EnteringConnecting;

#[derive(Default)]
pub struct ExitingInit;

//-------------------------------------------------------------------------------------------------------------------

/// System that broadcasts `T`.
pub fn broadcast_system<T: Default>(mut c: Commands)
{
    c.react().broadcast(T::default());
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct StateChangesPlugin;

impl Plugin for StateChangesPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(
            OnEnter(ClientFwState::Connecting),
            broadcast_system::<EnteringConnecting>,
        )
        .add_systems(OnExit(ClientFwState::Init), broadcast_system::<ExitingInit>);
    }
}

//-------------------------------------------------------------------------------------------------------------------
