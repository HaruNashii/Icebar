// ============ IMPORTS ============
use serde::{Serialize, Deserialize};
use iced::widget::image;





// ============ CRATES ============
use crate::modules::{clock::ClockData, media_player::MediaPlayerData, network::NetworkData, volume::VolumeData};
use crate::helpers::workspaces::WorkspaceData;





// ============ STRUCTS/ENUM'S ============
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Modules 
{
    CustomModule(usize),
    HyprWorkspaces,
    SwayWorkspaces,
    VolumeOutput,
    MediaPlayerMetaData,
    MediaPlayerButtons,
    VolumeInput,
    Network,
    Clock,
    Tray,
}

#[derive(Default, Clone)]
pub struct ModulesData
{
    pub tray_icons: Vec<(Option<image::Handle>, String)>,
    pub media_player_data: MediaPlayerData,
    pub workspace_data: WorkspaceData,
    pub active_modules: Vec<Modules>,
    pub network_data: NetworkData,
    pub volume_data: VolumeData,
    pub clock_data: ClockData
}
