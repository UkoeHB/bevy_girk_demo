//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_game_fw::*;
use bevy_kot_ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

//todo: consider converting this to an event, which can be responded to in a 'player click' plugin
fn handle_player_click_button(
    In(player_entity) : In<Entity>,
    mut players       : Query<&mut PlayerScore, With<PlayerId>>,
){
    let Ok(player_score) = players.get_component_mut::<PlayerScore>(player_entity)
    else { tracing::error!("handle player click button: unknown player entity"); return; };

    player_score.into_inner().increment();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn notify_request_rejected(
    In((client_id, request, reason)) : In<(ClientIdType, ClientRequest, RejectionReason)>,
    buffer                   : Res<GameMessageBuffer>
){
    buffer.send(
            GameMsg::RequestRejected{reason, request},
            vec![InfoAccessConstraint::Targets(vec![client_id])],
        );
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_mode_request(In(client_id): In<ClientIdType>, world: &mut World)
{
    syscall(world, client_id, notify_game_mode_single);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_player_input(
    In((
        player_entity,
        input
    ))    : In<(Entity, PlayerInput)>,
    world : &mut World,
){
    match input
    {
        PlayerInput::ClickButton => syscall(world, player_entity, handle_player_click_button),
    }
}

//-------------------------------------------------------------------------------------------------------------------
