use bevy_cobweb::prelude::*;
use bevy_simplenet::RequestSignal;

//-------------------------------------------------------------------------------------------------------------------

#[derive(ReactComponent, Debug, Clone)]
pub(crate) struct PendingRequest(RequestSignal);

impl PendingRequest
{
    pub(crate) fn new(new_req: RequestSignal) -> Self
    {
        Self(new_req)
    }
}

impl Deref for PendingRequest
{
    type Target = RequestSignal;
    fn deref(&self) -> &RequestSignal
    {
        &self.0
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when [`PendingRequest`] is added to the entity with tag component `T`.
#[derive(Default)]
pub(crate) struct RequestStarted<T>(PhantomData<T>);

/// Event broadcast when [`PendingRequest`] is removed from the entity with tag component `T`.
#[derive(Default)]
pub(crate) struct RequestEnded<T>(PhantomData<T>);

//-------------------------------------------------------------------------------------------------------------------

/// Spawns an entity with component `T` that will gain and lose `PendingRequest` components based on request
/// lifecycle.
pub(crate) fn spawn_request_entity<T: Component>(c: &mut Commands, tag: T) -> Entity
{
    let id = c.spawn(tag).id();
    c.react().on(entity_insertion::<PendingRequest>(id), |mut c: Commands| {
        c.react().broadcast(RequestStarted::<T>::default());
    });
    // Entity event reactors are a bit more efficient than entity-removal reactors.
    c.react().on((entity_event::<RequestSucceeded>(id), entity_event::<RequestFailed>(id)), |mut c: Commands| {
        c.react().broadcast(RequestEnded::<T>::default());
    });
}

//-------------------------------------------------------------------------------------------------------------------
