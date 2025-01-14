use bevy_cobweb::prelude::*;
use bevy_simplenet::RequestSignal;

//-------------------------------------------------------------------------------------------------------------------

/// PseudoState added/removed to an entity in response to RequestStarted/RequestEnded events.
const REQUEST_PENDING_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("RequestPending"));
const REQUEST_SUCCEEDED_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("RequestSucceeded"));
const REQUEST_FAILED_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("RequestFailed"));

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

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when [`PendingRequest`] is removed from the entity with tag component `T`.
#[derive(Default)]
pub(crate) enum RequestEnded<T>
{
    Success,
    Failure,
    /// Never constructed.
    Dummy(PhantomData<T>)
}

//-------------------------------------------------------------------------------------------------------------------

/// Spawns an entity with component `T` that will gain and lose `PendingRequest` components based on request
/// lifecycle.
pub(crate) fn spawn_request_entity<T: Component>(c: &mut Commands, tag: T) -> Entity
{
    let id = c.spawn(tag).id();
    c.react().on(entity_insertion::<PendingRequest>(id), |mut c: Commands| {
        c.react().broadcast(RequestStarted::<T>::default());
    });
    c.react().on(entity_event::<RequestSucceeded>(id), |mut c: Commands| {
        c.react().broadcast(RequestEnded::<T>::Success);
    });
    c.react().on(entity_event::<RequestFailed>(id), |mut c: Commands| {
        c.react().broadcast(RequestEnded::<T>::Failure);
    });
}

//-------------------------------------------------------------------------------------------------------------------

/// Adds/removes "Request{Pending/Succeeded/Failed}" pseudo-states from the scene node in response to
/// [`RequestStarted`]/[`RequestEnded`] events.
pub(crate) fn setup_ui_request_tracker<T>(h: &mut UiSceneHandle)
{
    h.reactor(broadcast::<RequestStarted<T>(), |id: TargetId, mut c: Commands, ps: PseudoStateParam| {
        ps.try_remove(&mut c, *id, REQUEST_SUCCEEDED_PSEUDOSTATE.clone());
        ps.try_remove(&mut c, *id, REQUEST_FAILED_PSEUDOSTATE.clone());

        ps.try_insert(&mut c, *id, REQUEST_PENDING_PSEUDOSTATE.clone());
    });
    h.reactor(
        broadcast::<RequestEnded<T>(),
        |id: TargetId, event: BroadcastEvent<RequestEnded<T>>, mut c: Commands, ps: PseudoStateParam| {
            ps.try_remove(&mut c, *id, REQUEST_PENDING_PSEUDOSTATE.clone());

            match event.try_read()? {
                RequestEnded::<T>::Success => {
                    ps.try_insert(&mut c, *id, REQUEST_SUCCEEDED_PSEUDOSTATE.clone());
                }
                RequestEnded::<T>::Failure => {
                    ps.try_insert(&mut c, *id, REQUEST_FAILED_PSEUDOSTATE.clone());
                }
                RequestEnded::<T>::Dummy(..) => unreachable!()
            }
            OK
        }
    );
}

//-------------------------------------------------------------------------------------------------------------------
