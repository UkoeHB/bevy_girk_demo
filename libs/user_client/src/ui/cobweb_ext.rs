use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_cobweb::react::ReactionTriggerBundle;
use bevy_cobweb_ui::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) trait CobwebReactExt
{
    /// Creates a reactor that enables/disables a UI node based on the given callback, which
    /// takes a system parameter the reactor should access.
    fn enable_if<T, C, P>(&mut self, triggers: T, callback: C) -> &mut Self
    where
        T: ReactionTriggerBundle,
        C: Fn(&P) -> bool + Send + Sync + 'static,
        P: SystemParam;
}

//-------------------------------------------------------------------------------------------------------------------

impl CobwebReactExt for UiBuilder<'_, Entity>
{
    fn enable_if<T, C, P>(&mut self, triggers: T, callback: C) -> &mut Self
    where
        T: ReactionTriggerBundle,
        C: Fn(&P) -> bool + Send + Sync + 'static,
        P: SystemParam,
    {
        self.update_on(
            triggers,
            move |id: TargetId, mut c: Commands, ps: PseudoStateParam, p: P| match (callback)(&p) {
                true => {
                    ps.try_enable(&mut c, *id);
                }
                false => {
                    ps.try_disable(&mut c, *id);
                }
            },
        )
    }
}

//-------------------------------------------------------------------------------------------------------------------
