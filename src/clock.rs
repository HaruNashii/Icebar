// ============ IMPORTS ============
use chrono::Local;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct ClockData
{
    pub current_time: String
}





// ============ FUNCTIONS ============
pub fn get_current_time(time_format: &str) -> String { Local::now().format(time_format).to_string() }
