use std::any::type_name;
use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_cobweb_ui::sickle::PseudoState;
use bevy_simplenet::RequestSignal;
use smol_str::SmolStr;

//-------------------------------------------------------------------------------------------------------------------

/// PseudoState added/removed to an entity in response to RequestStarted/RequestEnded events.
const REQUEST_PENDING_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("RequestPending"));
const REQUEST_SUCCEEDED_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("RequestSucceeded"));
const REQUEST_FAILED_PSEUDOSTATE: PseudoState = PseudoState::Custom(SmolStr::new_static("RequestFailed"));

//-------------------------------------------------------------------------------------------------------------------

#[derive(ReactComponent, Debug, Clone, Deref)]
pub(crate) struct PendingRequest(RequestSignal);

impl PendingRequest
{
    pub(crate) fn new(new_req: RequestSignal) -> Self
    {
        Self(new_req)
    }
}

impl From<RequestSignal> for PendingRequest
{
    fn from(signal: RequestSignal) -> Self
    {
        Self::new(signal)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Entity event sent to a pending request entity when a pending request succeeded.
pub(crate) struct RequestSucceeded;

//-------------------------------------------------------------------------------------------------------------------

/// Entity event sent to a pending request entity when a pending request failed.
pub(crate) struct RequestFailed;

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when [`PendingRequest`] is added to the entity with tag component `T`.
pub(crate) struct RequestStarted<T>(PhantomData<T>);

impl<T> Default for RequestStarted<T>
{
    fn default() -> Self
    {
        Self(PhantomData)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Event broadcast when [`PendingRequest`] is removed from the entity with tag component `T`.
///
/// Transforms the [`RequestSucceeded`] and [`RequestFailed`] entity events into a unified broadcast event.
pub(crate) enum RequestEnded<T>
{
    Success,
    Failure,
    /// Never constructed.
    #[allow(dead_code)]
    Dummy(PhantomData<T>),
}

//-------------------------------------------------------------------------------------------------------------------

/// System param for easier access to pending request entities.
#[derive(SystemParam)]
pub(crate) struct PendingRequestParam<'w, 's, T: Component>
{
    q: Query<'w, 's, (Entity, Option<&'static React<PendingRequest>>), With<T>>,
}

impl<'w, 's, T: Component> PendingRequestParam<'w, 's, T>
{
    pub(crate) fn entity(&self) -> Result<Entity, String>
    {
        let Ok((entity, _)) = self.q.get_single() else {
            return Err(
                format!("failed getting entity id for PendingRequest type {}; expected 1 entity, \
                found {} entities", type_name::<T>(), self.q.iter().count()),
            );
        };
        Ok(entity)
    }

    pub(crate) fn has_request(&self) -> bool
    {
        let Ok((_, maybe_req)) = self.q.get_single() else { return false };
        maybe_req.is_some()
    }

    pub(crate) fn request(&self) -> Option<(Entity, RequestSignal)>
    {
        let (entity, maybe_req) = self.q.get_single().ok()?;
        let req = maybe_req?;
        Some((entity, (***req).clone()))
    }

    pub(crate) fn add_request(&self, c: &mut Commands, new_req: impl Into<PendingRequest>)
    {
        let Ok(entity) = self.entity() else { return };
        let new_req: PendingRequest = new_req.into();
        c.react().insert(entity, new_req);
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Spawns an entity with component `T` that will gain and lose `PendingRequest` components based on request
/// lifecycle.
pub(crate) fn spawn_request_entity<T: Component>(c: &mut Commands, tag: T) -> Entity
{
    let id = c.spawn(tag).id();
    c.react()
        .on(entity_insertion::<PendingRequest>(id), |mut c: Commands| {
            c.react().broadcast(RequestStarted::<T>::default());
        });
    c.react()
        .on(entity_event::<RequestSucceeded>(id), |mut c: Commands| {
            c.react().broadcast(RequestEnded::<T>::Success);
        });
    c.react()
        .on(entity_event::<RequestFailed>(id), |mut c: Commands| {
            c.react().broadcast(RequestEnded::<T>::Failure);
        });
    id
}

//-------------------------------------------------------------------------------------------------------------------

/// Adds/removes "Request{Pending/Succeeded/Failed}" pseudo-states from the scene node in response to
/// [`RequestStarted`]/[`RequestEnded`] events.
pub(crate) fn setup_request_tracker<T: Send + Sync + 'static>(h: &mut UiSceneHandle)
{
    h.reactor(
        broadcast::<RequestStarted<T>>(),
        |id: TargetId, mut c: Commands, ps: PseudoStateParam| {
            ps.try_remove(&mut c, *id, REQUEST_SUCCEEDED_PSEUDOSTATE.clone());
            ps.try_remove(&mut c, *id, REQUEST_FAILED_PSEUDOSTATE.clone());

            ps.try_insert(&mut c, *id, REQUEST_PENDING_PSEUDOSTATE.clone());
        },
    );
    h.reactor(
        broadcast::<RequestEnded<T>>(),
        |id: TargetId, event: BroadcastEvent<RequestEnded<T>>, mut c: Commands, ps: PseudoStateParam| {
            ps.try_remove(&mut c, *id, REQUEST_PENDING_PSEUDOSTATE.clone());

            match event.try_read()? {
                RequestEnded::<T>::Success => {
                    ps.try_insert(&mut c, *id, REQUEST_SUCCEEDED_PSEUDOSTATE.clone());
                }
                RequestEnded::<T>::Failure => {
                    ps.try_insert(&mut c, *id, REQUEST_FAILED_PSEUDOSTATE.clone());
                }
                RequestEnded::<T>::Dummy(..) => (),
            }
            OK
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------
