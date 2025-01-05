
//-------------------------------------------------------------------------------------------------------------------

/// Loads base themes for all widgets.
pub fn load_common_widget_base_themes(builder: &mut UiBuilder<Entity>)
{
    BasicButton::load_base_theme(builder);
}

//-------------------------------------------------------------------------------------------------------------------

/// Plugin that sets up common widgets.
pub struct UiCommonWidgetsPlugin;

impl Plugin for UiCommonWidgetsPlugin
{
    fn build(&self, app: &mut App)
    {
        app.load_sheet("ui_common/widgets/manifest.load.json")
    }
}

//-------------------------------------------------------------------------------------------------------------------
