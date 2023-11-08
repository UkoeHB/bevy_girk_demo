//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Toggle a button's availability by showing/hiding a widget that blocks the button, and toggling the button label's
/// font color.
pub(crate) fn toggle_button_availability(
    In((
        enable,
        button_blocker,
        text_entity,
    ))             : In<(bool, Widget, Entity)>,
    mut uis        : Query<&mut UiTree, With<MainUI>>,
    mut text_query : Query<&mut Text>
){
    // toggle visibility of button blocker
    let Ok(mut ui) = uis.get_single_mut()
    else { tracing::error!("multiple uis detected in toggle button availability"); return; };
    let Ok(branch) = button_blocker.fetch_mut(&mut ui)
    else { tracing::error!("button blocker widget is missing"); return; };
    branch.set_visibility(!enable);

    // modify text color
    let Ok(mut text) = text_query.get_mut(text_entity)
    else { tracing::error!("text entity is missing in toggle button availability"); return; };
    let text_style = &mut text.sections[0].style;

    match enable
    {
        true => text_style.color = MISC_FONT_COLOR,
        false => text_style.color = DISABLED_BUTTON_FONT_COLOR,
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Write to the first text section in a text entity.
pub(crate) fn write_ui_text(world: &mut World, text_entity: Entity, writer: impl FnOnce(&mut String))
{
    let Some(mut entity) = world.get_entity_mut(text_entity)
    else { tracing::error!("failed writing ui text, missing entity"); return; };
    let Some(text) = entity.get_mut::<Text>()
    else { tracing::error!("failed writing ui text, entity missing Text component"); return; };

    let text_section = &mut text.into_inner().sections[0].value;
    text_section.clear();
    (writer)(text_section);
}

//-------------------------------------------------------------------------------------------------------------------
