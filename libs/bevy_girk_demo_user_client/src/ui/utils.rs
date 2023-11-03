//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_kot::ecs::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(SystemParam)]
pub(crate) struct UiBuilderCtx<'w, 's>
{
    pub(crate) rcommands    : ReactCommands<'w, 's>,
    pub(crate) asset_server : ResMut<'w, AssetServer>,

    main_ui: Query<'w, 's, &'static mut UiTree, With<MainUI>>
}

impl<'w, 's> UiBuilderCtx<'w, 's>
{
    pub(crate) fn commands<'a>(&'a mut self) -> &'a mut Commands<'w, 's>
    {
        self.rcommands.commands()
    }

    pub(crate) fn ui<'a>(&'a mut self) -> &'a mut UiTree
    {
        self.main_ui.single_mut().into_inner()
    }
}

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

/// Update the text in a UI element with a new value.
pub(crate) fn update_ui_text(In((text_entity, new_text)): In<(Entity, String)>, mut text_query: Query<&mut Text>)
{
    let Ok(mut text) = text_query.get_mut(text_entity)
    else { tracing::error!(?new_text, "text entity is missing for update ui text"); return; };
    text.sections[0].value = new_text;
}

//-------------------------------------------------------------------------------------------------------------------
