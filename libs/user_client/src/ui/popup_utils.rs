use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::ClientAppState;

//-------------------------------------------------------------------------------------------------------------------

/// Creates a reactor system that ties opening and closing a popup to a reactive resource `T`.
pub(crate) fn setup_reactres_managed_popup<T: ReactResource, R: CobwebResult>(
    should_open: impl Fn(&T) -> bool + Send + Sync + 'static,
    scene_ref: (&'static str, &'static str),
    build_fn: fn(&mut UiSceneHandle) -> R,
) -> impl IntoSystem<(), DropErr, ()> + Send + Sync + 'static
{
    IntoSystem::into_system(
        move |//
        mut popup: Local<Option<Entity>>,
        mut c: Commands,
        mut s: SceneBuilder,
        res: ReactRes<T>//
    | -> DropErr
    {
        let should_open = (should_open)(&res);
        if should_open == popup.is_some() { return DONE }

        match should_open {
            true => {
                c.ui_root().spawn_scene(scene_ref, &mut s, |h| {
                    *popup = Some(h.id());

                    h.insert(StateScoped(ClientAppState::Client));
                    (build_fn)(h)
                });
            }
            false => {
                let entity = popup.take().result()?;
                c.get_entity(entity)?.despawn();
            }
        }

        DONE
    },
    )
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn setup_broadcast_popup<T: Send + Sync + 'static, R: CobwebResult>(
    scene_ref: (&'static str, &'static str),
    build_fn: fn(&T, &mut UiSceneHandle) -> R,
) -> impl IntoSystem<(), WarnErr, ()> + Send + Sync + 'static
{
    IntoSystem::into_system(
        move |//
        event: BroadcastEvent<T>,
        mut c: Commands,
        mut s: SceneBuilder//
    | -> WarnErr
    {
        let event = event.try_read()?;

        c.ui_root().spawn_scene(scene_ref, &mut s, |h| {
            h.insert(StateScoped(ClientAppState::Client));
            (build_fn)(event, h)
        });

        OK
    },
    )
}

//-------------------------------------------------------------------------------------------------------------------
