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
    let Ok(mut player_score) = players.get_mut(player_entity) else {
        tracing::error!("handle player click button: unknown player entity");
        return;
    };

    player_score.increment();
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn notify_request_rejected(
    In((client_id, request, reason)): In<(ClientId, ClientRequest, RejectionReason)>,
    mut sender: GameSender,
)
{
    sender.send_to_client(GameMsg::RequestRejected { reason, request }, client_id.get());
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_game_state_request(In(client_id): In<ClientId>, world: &mut World)
{
    world.syscall(client_id, notify_game_state_single);
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn handle_player_input(In((player_entity, input)): In<(Entity, PlayerInput)>, world: &mut World)
{
    match input {
        PlayerInput::ClickButton => world.syscall(player_entity, handle_player_click_button),
    }
}

//-------------------------------------------------------------------------------------------------------------------
