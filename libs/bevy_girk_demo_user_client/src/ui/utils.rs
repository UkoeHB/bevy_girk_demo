//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

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
