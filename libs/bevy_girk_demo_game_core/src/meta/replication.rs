//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_game_fw::*;
use bevy_kot_ecs::*;
use bevy_replicon::bincode;
use bevy_replicon::bincode::*;
use bevy_replicon::prelude::*;
use bevy_replicon::replicon_core::replication_rules::*;
use serde::de::DeserializeOwned;

//standard shortcuts
use std::io::Cursor;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn update_react_component<C: ReactComponent>(
    In((component, entity)) : In<(C, Entity)>,
    mut rcommands           : ReactCommands,
    mut query               : Query<&mut React<C>>,
){
    let Ok(mut existing) = query.get_mut(entity)
    else
    {
        rcommands.insert(entity, component);
        return;
    };
    *existing.get_mut(&mut rcommands) = component;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn deserialize_react_component<C: Component + ReactComponent + DeserializeOwned>(
    entity      : &mut EntityWorldMut,
    _entity_map : &mut ServerEntityMap,
    cursor      : &mut Cursor<&[u8]>,
    _tick       : RepliconTick,
) -> bincode::Result<()>
{
    let component: C = DefaultOptions::new().deserialize_from(cursor)?;
    let entity_id = entity.id();
    entity.world_scope(move |world| syscall(world, (component, entity_id), update_react_component));

    Ok(())
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn remove_react_component<C: Component + ReactComponent>(entity: &mut EntityWorldMut, _tick: RepliconTick)
{
    entity.remove::<React<C>>();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Initializes all game components that may be replicated (including game framework components).
///
/// Depends on `bevy_replicon::replication_core::ReplicationCorePlugin`.
#[bevy_plugin]
pub fn GameReplicationPlugin(app: &mut App)
{
    app.replicate::<PlayerId>()
        .replicate_with::<PlayerName>(
            serialize_component::<PlayerName>,
            deserialize_react_component::<PlayerName>,
            remove_react_component::<PlayerName>)
        .replicate_with::<PlayerScore>(
            serialize_component::<PlayerScore>,
            deserialize_react_component::<PlayerScore>,
            remove_react_component::<PlayerScore>)
        .replicate::<GameInitProgress>();
}

//-------------------------------------------------------------------------------------------------------------------
