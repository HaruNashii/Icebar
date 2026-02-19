use chrono::Local;




#[derive(Default, Clone)]
pub struct ClockData
{
    pub current_time: String
}





pub fn get_current_time() -> String
{
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    time_str
}
