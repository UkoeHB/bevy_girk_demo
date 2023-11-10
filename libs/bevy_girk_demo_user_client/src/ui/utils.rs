//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_kot::prelude::*;
use bevy_lunex::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Toggle a button's availability by showing/hiding a widget that blocks the button, and toggling the button label's
/// font color.
pub fn toggle_button_availability(
    In((
        enable,
        button_blocker,
        text_entity,
    ))             : In<(bool, Widget, Entity)>,
    mut uis        : Query<&mut UiTree, With<MainUI>>,
    mut text_query : Query<&mut Text>,
    style_stack    : Res<StyleStackRes<MainUI>>,
){
    // toggle visibility of button blocker
    let Ok(mut ui) = uis.get_single_mut()
    else { tracing::error!("multiple uis detected in toggle button availability"); return; };
    let Ok(branch) = button_blocker.fetch_mut(&mut ui)
    else { tracing::error!("button blocker widget is missing"); return; };
    branch.set_visibility(!enable);

    // get text
    let Ok(mut text) = text_query.get_mut(text_entity)
    else { tracing::error!("text entity is missing in toggle button availability"); return; };
    let text_style = &mut text.sections[0].style;

    // get style for the text based on availability
    let availability_style = style_stack.get::<BasicButtonAvailability>().unwrap();

    // update text color
    match enable
    {
        true => text_style.color = availability_style.active,
        false => text_style.color = availability_style.inactive,
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Write to the first text section in a text entity.
pub fn write_ui_text(world: &mut World, text_entity: Entity, writer: impl FnOnce(&mut String))
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

/// Helper system param for accessing text components.
#[derive(SystemParam)]
pub struct TextHandle<'w, 's>
{
    text: Query<'w, 's, &'static mut Text>,
}

impl<'w, 's> TextHandle<'w, 's>
{
    /// Get the text on a text section on an entity.
    ///
    /// Returns `Err` if the text section could not be found or the text is empty.
    pub fn text(&mut self, text_entity: Entity, section: usize) -> Result<&mut String, ()>
    {
        let Ok(text) = self.text.get_mut(text_entity) else { return Err(()); };
        let Some(section) = text.into_inner().sections.get_mut(section) else { return Err(()); };
        Ok(&mut section.value)
    }

    /// Get the style on a text section on an entity.
    ///
    /// Returns `Err` if the text section could not be found or the text is empty.
    pub fn style(&mut self, text_entity: Entity, section: usize) -> Result<&mut TextStyle, ()>
    {
        let Ok(text) = self.text.get_mut(text_entity) else { return Err(()); };
        let Some(section) = text.into_inner().sections.get_mut(section) else { return Err(()); };
        Ok(&mut section.style)
    }
}

//-------------------------------------------------------------------------------------------------------------------
