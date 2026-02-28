// ============ IMPORTS ============
use serde::{Serialize, Deserialize};
use iced::widget::image;





// ============ CRATES ============
use crate::{helpers::workspaces::WorkspaceData, modules::{clock::ClockData, network::NetworkData, volume::VolumeData}};





// ============ STRUCTS/ENUM'S ============
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Modules 
{
    CustomModule(usize),
    HyprWorkspaces,
    SwayWorkspaces,
    VolumeOutput,
    VolumeInput,
    Network,
    Clock,
    Tray,
}

#[derive(Default, Clone)]
pub struct ModulesData
{
    pub tray_icons: Vec<(Option<image::Handle>, String)>,
    pub workspace_data: WorkspaceData,
    pub active_modules: Vec<Modules>,
    pub network_data: NetworkData,
    pub volume_data: VolumeData,
    pub clock_data: ClockData
}
