use chrono::Local;




#[derive(Default, Clone)]
pub struct ClockData
{
    pub current_time: String
}





pub fn get_current_time(time_format: &str) -> String
{
    let now = Local::now();
    let time_str = now.format(time_format).to_string();
    time_str
}
