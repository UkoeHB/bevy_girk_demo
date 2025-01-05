use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_game_fw::*;
use bevy_replicon::prelude::*;
use bevy_replicon_attributes::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

//todo: consider converting this to an event, which can be responded to in a 'player click' plugin
fn handle_player_click_button(In(player_entity): In<Entity>, mut players: Query<&mut PlayerScore, With<PlayerId>>)
{
    let Ok(player_score) = players.get_mut(player_entity) else {
        tracing::error!("handle player click button: unknown player entity");
        return;
    };

    player_score.into_inner().increment();
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn notify_request_rejected(
    In((client_id, request, reason)): In<(ClientId, ClientRequest, RejectionReason)>,
    mut sender: GameMessageSender,
    attributes: ClientAttributes,
)
{
    sender.send(
        &attributes,
        GameMsg::RequestRejected { reason, request },
        vis!(Client(client_id)),
    );
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_mode_request(In(client_id): In<ClientId>, world: &mut World)
{
    syscall(world, client_id, notify_game_mode_single);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_player_input(In((player_entity, input)): In<(Entity, PlayerInput)>, world: &mut World)
{
    match input {
        PlayerInput::ClickButton => syscall(world, player_entity, handle_player_click_button),
    }
}

//-------------------------------------------------------------------------------------------------------------------
