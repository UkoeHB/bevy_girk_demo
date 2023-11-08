//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub struct BasicText
{
    pub color: Color,
    pub font: &'static str,
}

impl Default for BasicText
{
    fn default() -> Self
    {
        Self{
            color: Color::BLACK,
            font: BASIC_FONT,
        }
    }
}

pub fn basic_text_default_light_style() -> BasicText
{
    BasicText{
            color: Color::WHITE,
            font: BASIC_FONT,
        }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub struct BasicButton
{
    pub default_img: (&'static str, Vec2),
    pub pressed_img: (&'static str, Vec2),

    pub default_img_color: Color,
    pub pressed_img_color: Color,

    pub text: BasicText,
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
            text: basic_button_default_text_style(),
        }
    }
}

pub fn basic_button_default_text_style() -> BasicText
{
    basic_text_default_light_style()
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub struct BasicButtonAvailability
{
    pub active: Color,
    pub inactive: Color,
}

impl Default for BasicButtonAvailability
{
    fn default() -> Self
    {
        Self{
            active: basic_button_default_text_style().color,
            inactive: Color::GRAY,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone)]
pub struct BasicPopup
{
    pub button: BasicButton,
    pub background: (&'static str, Vec2),
    pub window: (&'static str, Vec2),
    pub window_color: Color,
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

pub fn basic_popup_default_button_style() -> BasicButton
{
    BasicButton::default()
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(StyleBundle, Default)]
pub struct PrefabStyles
{
    basic_text: BasicText,
    basic_button: BasicButton,
    basic_button_availability: BasicButtonAvailability,
    basic_popup: BasicPopup,
}

//-------------------------------------------------------------------------------------------------------------------
