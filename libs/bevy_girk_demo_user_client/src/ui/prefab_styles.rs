//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub(crate) struct BasicText
{
    pub(crate) color: Color,
    pub(crate) font: &'static str,
}

impl Default for BasicText
{
    fn default() -> Self
    {
        Self{
            color: MISC_FONT_COLOR,
            font: MISC_FONT,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub(crate) struct BasicButton
{
    pub(crate) default_img: (&'static str, Vec2),
    pub(crate) pressed_img: (&'static str, Vec2),

    pub(crate) default_img_color: Color,
    pub(crate) pressed_img_color: Color,

    pub(crate) text: BasicText,
}

impl Default for BasicButton
{
    fn default() -> Self
    {
        Self{
            default_img: MISC_BUTTON,
            pressed_img: MISC_BUTTON,
            default_img_color: Color::GRAY,
            pressed_img_color: Color::DARK_GRAY,
            text: BasicText::default(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub(crate) struct BasicPopup
{
    pub(crate) button: BasicButton,
    pub(crate) background: (&'static str, Vec2),
    pub(crate) window: (&'static str, Vec2),
    pub(crate) window_color: Color,
}

impl Default for BasicPopup
{
    fn default() -> Self
    {
        Self{
            button: BasicButton::default(),
            background: FILM,
            window: BOX,
            window_color: Color::AZURE,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(StyleBundle, Default)]
pub(crate) struct PrefabStyles
{
    basic_text: BasicText,
    basic_button: BasicButton,
    basic_popup: BasicPopup,
}

//-------------------------------------------------------------------------------------------------------------------
