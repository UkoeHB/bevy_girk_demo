use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) trait CobwebReactExt
{
    /// Creates a reactor that enables/disables a UI node based on the given callback, which
    /// takes a system parameter the reactor should access.
    fn enable_if<T, C, M>(&mut self, triggers: T, callback: C) -> &mut Self
    where
        T: ReactionTriggerBundle,
        C: IntoSystem<(), bool, M> + Send + Sync + 'static;
}

//-------------------------------------------------------------------------------------------------------------------

impl CobwebReactExt for UiBuilder<'_, Entity>
{
    fn enable_if<T, C, M>(&mut self, triggers: T, callback: C) -> &mut Self
    where
        T: ReactionTriggerBundle,
        C: IntoSystem<(), bool, M> + Send + Sync + 'static,
    {
        let mut system = RawCallbackSystem::new(callback);
        self.update_on(
            triggers,
            move |id: TargetId, w: &mut World| {
                let r = system.run(w, ());
                w.syscall((*id, r), |In((id, r)): In<(Entity, bool)>,  mut c: Commands, ps: PseudoStateParam| {
                    match r {
                        true => {
                            ps.try_enable(&mut c, id);
                        }
                        false => {
                            ps.try_disable(&mut c, id);
                        }
                    }
                });
            }
        )
    }
}

//-------------------------------------------------------------------------------------------------------------------
