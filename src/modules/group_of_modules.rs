// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use iced::{Border, border::Radius, widget::container};



// ============ CRATES ============
use crate::helpers::color::ColorType;
use crate::modules::data::Modules;



// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GroupOfModulesGroupsConfig
{
    pub groups: Vec<GroupOfModulesConfig>
}



#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GroupOfModulesConfig
{
    pub modules:          Vec<Modules>,
    pub spacing_inside:   u32,
    pub padding:          u16,
    pub background_color: ColorType,
    pub border_color:     ColorType,
    pub border_size:      f32,
    pub border_radius:    [f32; 4]
}

impl Default for GroupOfModulesConfig
{
    fn default() -> Self
    {
        Self
        {
            modules:          vec![],
            spacing_inside:   0,
            padding:          0,
            background_color: ColorType::RGB([36, 36, 36]),
            border_color:     ColorType::RGB([60, 60, 60]),
            border_size:      0.0,
            border_radius:    [0., 0., 0., 0.]
        }
    }
}



// ============ FUNCTIONS ============
pub fn group_container_style(group: &GroupOfModulesConfig) -> impl Fn(&iced::Theme) -> container::Style
{
    // Compute everything from the borrow NOW, before the closure is stored.
    // The returned closure has no lifetime parameter so it does not carry a
    // borrow of AppData into iced_layershell's transmute-to-'static path.
    let style = container::Style
    {
        background: Some(iced::Background::Color(group.background_color.to_iced_color())),
        border: Border
        {
            color:  group.border_color.to_iced_color(),
            width:  group.border_size,
            radius: Radius
            {
                top_left:     group.border_radius[0],
                top_right:    group.border_radius[1],
                bottom_left:  group.border_radius[2],
                bottom_right: group.border_radius[3]
            }
        },
        ..Default::default()
    };

    move |_theme: &iced::Theme| style
}



// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn group_of_modules_default_modules_is_empty()
    {
        assert!(GroupOfModulesConfig::default().modules.is_empty());
    }

    #[test]
    fn group_of_modules_default_spacing_inside_is_zero()
    {
        assert_eq!(GroupOfModulesConfig::default().spacing_inside, 0);
    }

    #[test]
    fn group_of_modules_default_padding_is_zero()
    {
        assert_eq!(GroupOfModulesConfig::default().padding, 0);
    }

    #[test]
    fn group_of_modules_default_border_size_is_zero()
    {
        assert_eq!(GroupOfModulesConfig::default().border_size, 0.0);
    }

    #[test]
    fn group_of_modules_config_default_groups_is_empty()
    {
        assert!(GroupOfModulesGroupsConfig::default().groups.is_empty());
    }

    #[test]
    fn group_of_modules_default_border_radius_is_all_zeros()
    {
        assert_eq!(GroupOfModulesConfig::default().border_radius, [0., 0., 0., 0.]);
    }
}
