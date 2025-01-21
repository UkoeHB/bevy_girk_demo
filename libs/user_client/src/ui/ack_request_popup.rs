use std::fmt::Write;

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn build_ack_popup(h: &mut UiSceneHandle) -> WarnErr
{
    let accept_id = h.get_entity("accept_button")?;
    let cancel_id = h.get_entity("cancel_button")?;
    h.get("content::timer::text").update_on(
        resource_mutation::<AckRequestData>(),
        |id: TargetId, mut e: TextEditor, data: ReactRes<AckRequestData>| {
            let secs_remaining = data.time_remaining_for_display().as_secs();
            write_text!(e, *id, "{}", secs_remaining);
        },
    );
    h.get("accept_button")
        .on_pressed(move |mut c: Commands, ps: PseudoStateParam| {
            c.syscall((), send_lobby_ack);
            ps.try_disable(&mut c, accept_id);
            // Don't disable reject button because we can reject after acking if the ack request isn't completely
            // acked yet.
        });
    h.get("cancel_button")
        .on_pressed(move |mut c: Commands, ps: PseudoStateParam| {
            c.syscall((), send_lobby_nack);
            ps.try_disable(&mut c, accept_id); // Also disable accept button since nacking takes precedence.
            ps.try_disable(&mut c, cancel_id);
        });

    OK
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct UiAckLobbyPopupPlugin;

impl Plugin for UiAckLobbyPopupPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_reactor(
            resource_mutation::<AckRequestData>(),
            setup_reactres_managed_popup(
                |data: &AckRequestData| data.is_set(),
                ("ui.user", "ack_popup"),
                build_ack_popup,
            ),
        );
    }
}

//-------------------------------------------------------------------------------------------------------------------
