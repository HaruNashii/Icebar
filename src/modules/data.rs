// ============ IMPORTS ============
use iced::widget::image;





// ============ CRATES ============
use crate::{helpers::workspaces::WorkspaceData, modules::{clock::ClockData, volume::VolumeData}};





// ============ FUNCTIONS ============
#[derive(Default, Clone)]
pub struct Modules 
{
    pub active_modules: Vec<String>,
}

#[derive(Default, Clone)]
pub struct ModulesData
{
    pub tray_icons: Vec<(Option<image::Handle>, String)>,
    pub volume_data: VolumeData,
    pub clock_data: ClockData,
    pub workspace_data: WorkspaceData
}
