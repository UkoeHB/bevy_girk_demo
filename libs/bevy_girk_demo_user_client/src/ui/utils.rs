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

    /// Overwrite the text on a text section on an entity.
    ///
    /// Returns `Err` if the text section could not be accessed or if the writer fails.
    pub fn write<E>(
        &mut self,
        text_entity: Entity,
        section: usize,
        writer: impl FnOnce(&mut String) -> Result<(), E>
    ) -> Result<(), ()>
    {
        let text = self.text(text_entity, section)?;
        text.clear();
        (writer)(text).map_err(|_| ())
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

/// Helper system param for interacting with a `UiTree`.
//todo: handle multiple uis (pass in UI entity)?
#[derive(SystemParam)]
pub struct UiUtils<'w, 's, U: LunexUI>
{
    pub builder: UiBuilder<'w, 's, U>,
    pub text: TextHandle<'w, 's>,
}

impl<'w, 's, U: LunexUI> UiUtils<'w, 's, U>
{
    /// Toggle between two sets of widgets.
    pub fn toggle<const ON: usize, const OFF: usize>(
        &mut self,
        on_widgets  : &[Widget; ON],
        off_widgets : &[Widget; OFF],
    ){
        // get ui
        let ui = self.builder.tree();

        // set widget visibility: on
        for on_widget in on_widgets
        {
            let Ok(on_widget_branch) = on_widget.fetch_mut(ui) else { continue; };
            on_widget_branch.set_visibility(true);
        }

        // set widget visibility: off
        for off_widget in off_widgets
        {
            let Ok(off_widget_branch) = off_widget.fetch_mut(ui) else { continue; };
            off_widget_branch.set_visibility(false);
        }
    }

    /// Toggle a button's availability.
    /// - Shows/hides a widget that blocks the button.
    /// - Toggles the button label's font color.
    pub fn toggle_basic_button(&mut self, enable: bool, button_blocker: &Widget, text_entity: Entity)
    {
        // toggle visibility of button blocker
        let ui = self.builder.tree();
        let Ok(branch) = button_blocker.fetch_mut(ui)
        else { tracing::error!("button blocker widget is missing"); return; };
        branch.set_visibility(!enable);

        // get text
        let Ok(text_style) = self.text.style(text_entity, 0)
        else { tracing::error!("text entity is missing in toggle button availability"); return; };

        // get style for the text based on availability
        let availability_style = self.builder.get::<BasicButtonAvailability>().unwrap();

        // update text color
        match enable
        {
            true => text_style.color = availability_style.active,
            false => text_style.color = availability_style.inactive,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
