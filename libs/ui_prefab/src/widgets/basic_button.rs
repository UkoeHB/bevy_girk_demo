
//-------------------------------------------------------------------------------------------------------------------

#[derive(TypeName)]
pub struct BasicButtonText;

/// Marker component for the basic button theme.
#[derive(Component, DefaultTheme, Copy, Clone, Debug)]
pub struct BasicButton
{
    pub text_entity: Entity,
}

impl BasicButton
{
    fn load_base_theme(builder: &mut UiBuilder<Entity>)
    {
        let theme = LoadableRef::new(BasicButtonBuilder::widget_file(), "theme");
        builder.load_theme::<BasicButton>(theme.e("core"));
        builder.load_subtheme::<BasicButton, BasicButtonText>(theme.e("text"));
    }
}

impl UiContext for BasicButton
{
    fn get(&self, target: &str) -> Result<Entity, String>
    {
        match target {
            BasicButtonText::NAME => Ok(self.text_entity),
            _ => Err(format!("unknown UI context {target} for {}", type_name::<Self>())),
        }
    }
    fn contexts(&self) -> Vec<&'static str>
    {
        vec![BasicButtonText::NAME]
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Builder for [`BasicButton`] widgets.
pub struct BasicButtonBuilder
{
    text: String,
    structure: Option<LoadableRef>,
    core_theme: Option<LoadableRef>,
    text_theme: Option<LoadableRef>,
}

impl BasicButtonBuilder
{
    /// Makes a new widget builder.
    pub fn new(text: impl Into<String>) -> Self
    {
        Self { text: text.into(), ..Default::default() }
    }

    /// Sets the path to where the widget structure is defined.
    ///
    /// Defaults to the pre-defined structure.
    pub fn structure(self, path: LoadableRef) -> Self
    {
        self.structure = Some(path);
        self
    }

    /// Sets the path to where the theme of the core entity in the widget structure is defined.
    ///
    /// Defaults to the pre-defined theme.
    pub fn core_theme(self, path: LoadableRef) -> Self
    {
        self.core_theme = Some(path);
        self
    }

    /// Sets the path to where the theme of the text entity in the widget structure is defined.
    ///
    /// Defaults to the pre-defined theme.
    pub fn text_theme(self, path: LoadableRef) -> Self
    {
        self.text_theme = Some(path);
        self
    }

    /// Gets the file where the pre-defined widget structure and theme are located.
    pub fn widget_file() -> &'static str
    {
        "common.widgets.basic_button"
    }

    /// Builds the widget.
    ///
    /// If you do not define any theming then theming for this widget from the nearest ancestor where it is loaded
    /// will be used.
    ///
    /// Returns a `UiBuilder` for the core entity of this widget where the widget component is inserted, and where
    /// interactions will be detected.
    pub fn build<'a>(self, node: &'a mut UiBuilder<Entity>) -> UiBuilder<'a, Entity>
    {
        let mut core_entity = Entity::PLACEHOLDER;
        let mut text_entity = Entity::PLACEHOLDER;

        let structure = self.structure.unwrap_or(LoadableRef::new(Self::widget_file(), "structure"));

        node.load_with_theme::<BasicButton>(structure.e("core"), &mut core_entity, |core, path| {
            if let Some(extra_theme) = self.core_theme {
                core.load_theme::<BasicButton>(extra_theme);
            }

            core.load_with_subtheme::<BasicButtonText>(path.e("text"), &mut text_entity, |text, _| {
                if let Some(extra_theme) = self.text_theme {
                    text.load_subtheme::<BasicButtonText>(extra_theme);
                }

                // Note: The text needs to be updated on load otherwise it may be overwritten.
                let text_val = self.text;
                text.update_on((), |id|
                    move |mut e: TextEditor| {
                        e.write(id, |t| write!(t, "{}", text_val.as_str()));
                    }
                );
            });

            core.insert(BasicButton{
                text_entity
            });
        });

        // Return UiBuilder for root of button where interactions will be detected.
        node.commands().ui_builder(core_entity)
    }
}

//-------------------------------------------------------------------------------------------------------------------
