use std::io::Cursor;

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_fn_plugin::*;
use bevy_replicon::bincode;
use bevy_replicon::core::ctx::WriteCtx;
use bevy_replicon::core::replication_registry::command_fns::default_remove;
use bevy_replicon::core::replication_registry::rule_fns::RuleFns;
use bevy_replicon::prelude::*;
use bevy_replicon_repair::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn update_react_component<C: ReactComponent>(
    In((component, entity)): In<(C, Entity)>,
    mut c: Commands,
    mut query: Query<&mut React<C>>,
)
{
    let Ok(mut existing) = query.get_mut(entity) else {
        c.react().insert(entity, component);
        return;
    };
    *existing.get_mut(&mut c) = component;
}

//-------------------------------------------------------------------------------------------------------------------

fn write_react_component<C: Component + ReactComponent + DeserializeOwned>(
    ctx: &mut WriteCtx,
    rule_fns: &RuleFns<C>,
    entity: &mut EntityMut,
    cursor: &mut Cursor<&[u8]>,
) -> bincode::Result<()>
{
    let component: C = rule_fns.deserialize(ctx, cursor)?;
    let entity_id = entity.id();
    ctx.commands
        .add(move |world: &mut World| syscall(world, (component, entity_id), update_react_component));

    Ok(())
}

//-------------------------------------------------------------------------------------------------------------------

//todo: move to bevy_girk
trait ReplicateRepairReactExt
{
    fn replicate_repair_react<C>(&mut self) -> &mut Self
    where
        C: Component + ReactComponent + Serialize + DeserializeOwned;
}

impl ReplicateRepairReactExt for App
{
    fn replicate_repair_react<C>(&mut self) -> &mut Self
    where
        C: Component + ReactComponent + Serialize + DeserializeOwned,
    {
        self.set_command_fns::<C>(write_react_component::<C>, default_remove::<React<C>>)
            .replicate_repair_with::<C>(RuleFns::<C>::default(), repair_component::<React<C>>)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Initializes all game components that may be replicated (including game framework components).
///
/// Depends on `bevy_replicon::replication_core::ReplicationCorePlugin`.
#[bevy_plugin]
pub fn GameReplicationPlugin(app: &mut App)
{
    app.replicate_repair::<PlayerId>()
        .replicate_repair_react::<PlayerName>()
        .replicate_repair_react::<PlayerScore>();
}

//-------------------------------------------------------------------------------------------------------------------
