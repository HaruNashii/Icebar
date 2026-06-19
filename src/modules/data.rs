// ============ IMPORTS ============
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use iced::widget::image;





// ============ CRATES ============
use crate::modules::{clock::ClockData, custom_modules::CustomModuleData, focused_window::FocusedWindowData, image::ImageData, media_player::MediaPlayerData, network::NetworkData, power_profile::PowerProfileData, volume::VolumeData, workspaces::WorkspaceData};
use crate::windows::{calendar::CalendarData, volume_mixer::{VolumeMixerData, MixerState}, media_player_window::MediaPlayerWindowData};





// ============ ENUM/STRUCT, ETC ============
#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Modules
{
    Image(usize),
    CustomModule(usize),
    #[serde(alias = "GroupOfModules")]
    Group(usize),
    PlasmaWorkspaces,
    PowerProfile,
    HyprWorkspaces,
    NiriWorkspaces,
    SwayWorkspaces,
    CosmicWorkspaces,
    VolumeOutput,
    MediaPlayerMetaData,
    MediaPlayerButtons,
    FocusedWindowSway,
    FocusedWindowHypr,
    FocusedWindowNiri,
    FocusedWindowCosmic,
    VolumeInput,
    Network,
    CpuTemp,
    Clock,
    Tray,
    Disk,
    Cpu,
    Ram
}

#[derive(Default, Clone)]
pub struct ModulesData
{
    pub tray_icons: Vec<(Option<image::Handle>, String)>,
    pub focused_window_data: FocusedWindowData,
    pub custom_module_data: CustomModuleData,
    pub power_profile_data: PowerProfileData,
    pub media_player_data: MediaPlayerData,
    pub active_modules: HashSet<Modules>,
    pub plasma_desktop_ids: Vec<String>,
    pub workspace_data: WorkspaceData,
    pub calendar_data: CalendarData,
    pub network_data: NetworkData,
    pub volume_data: VolumeData,
    pub clock_data: ClockData,
    pub image_data: ImageData,
    pub cpu_text: String,
    pub ram_text: String,
    pub disk_text: String,
    pub cpu_temp_text: String,
    pub volume_mixer_data: VolumeMixerData,
    pub mixer_state: MixerState,
    pub media_player_window_data: MediaPlayerWindowData,
    pub album_art: Option<image::Handle>,
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn modules_same_variants_are_equal()
    {
        assert_eq!(Modules::Disk,                Modules::Disk);
        assert_eq!(Modules::Clock,               Modules::Clock);
        assert_eq!(Modules::Network,             Modules::Network);
        assert_eq!(Modules::Tray,                Modules::Tray);
        assert_eq!(Modules::HyprWorkspaces,      Modules::HyprWorkspaces);
        assert_eq!(Modules::SwayWorkspaces,      Modules::SwayWorkspaces);
        assert_eq!(Modules::NiriWorkspaces,      Modules::NiriWorkspaces);
        assert_eq!(Modules::CosmicWorkspaces,    Modules::CosmicWorkspaces);
        assert_eq!(Modules::FocusedWindowCosmic, Modules::FocusedWindowCosmic);
        assert_eq!(Modules::VolumeOutput,        Modules::VolumeOutput);
        assert_eq!(Modules::VolumeInput,         Modules::VolumeInput);
        assert_eq!(Modules::MediaPlayerMetaData, Modules::MediaPlayerMetaData);
        assert_eq!(Modules::MediaPlayerButtons,  Modules::MediaPlayerButtons);
    }

    #[test]
    fn modules_different_variants_are_not_equal()
    {
        assert_ne!(Modules::Clock,          Modules::Network);
        assert_ne!(Modules::Tray,           Modules::VolumeOutput);
        assert_ne!(Modules::HyprWorkspaces,   Modules::SwayWorkspaces);
        assert_ne!(Modules::SwayWorkspaces,   Modules::NiriWorkspaces);
        assert_ne!(Modules::NiriWorkspaces,   Modules::CosmicWorkspaces);
        assert_ne!(Modules::CosmicWorkspaces, Modules::HyprWorkspaces);
    }

    #[test]
    fn modules_custom_module_same_index_equal()
    {
        assert_eq!(Modules::CustomModule(0), Modules::CustomModule(0));
        assert_eq!(Modules::CustomModule(9), Modules::CustomModule(9));
    }

    #[test]
    fn modules_custom_module_different_index_not_equal()
    {
        assert_ne!(Modules::CustomModule(0), Modules::CustomModule(1));
    }

    #[test]
    fn modules_custom_module_not_equal_to_non_custom_variants()
    {
        assert_ne!(Modules::CustomModule(0), Modules::Clock);
        assert_ne!(Modules::CustomModule(0), Modules::Tray);
    }

    #[test]
    fn modules_data_default_tray_icons_is_empty()
    {
        assert!(ModulesData::default().tray_icons.is_empty());
    }

    #[test]
    fn modules_data_default_active_modules_is_empty()
    {
        assert!(ModulesData::default().active_modules.is_empty());
    }

    #[test]
    fn modules_data_default_clock_time_is_empty_string()
    {
        assert_eq!(ModulesData::default().clock_data.current_time, "");
    }

    #[test]
    fn modules_data_default_network_id_is_empty_string()
    {
        assert_eq!(ModulesData::default().network_data.id, "");
    }

    #[test]
    fn modules_data_default_media_metadata_is_empty_string()
    {
        assert_eq!(ModulesData::default().media_player_data.metadata, "");
    }

    #[test]
    fn modules_data_default_workspace_visible_is_empty()
    {
        assert!(ModulesData::default().workspace_data.visible_workspaces.is_empty());
    }

    #[test]
    fn modules_data_default_workspace_current_is_zero()
    {
        assert_eq!(ModulesData::default().workspace_data.current_workspace, 0);
    }
}
