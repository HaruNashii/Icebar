use display_info::DisplayInfo;





pub fn get_monitor_res(option_display: Option<String>) -> (u32, u32)
{
    let display_infos = DisplayInfo::all().unwrap();
    if let Some(display) = option_display
    {
        for vec_display in &display_infos
        {
            if display == vec_display.name
            {
                println!("\n=== Display Configuration ===");
                println!("Display Parsed With Ron: {}", display);
                println!("Display Parsed With DisplayInfo: {}", vec_display.name);
                return (vec_display.width, vec_display.height);

            }
        }
    }

    if let Some(display_info) = display_infos.into_iter().next()
    {
        println!("\n=== Display Configuration ===");
        println!("Warning!!!: No Display Or Non-Existent Display Parsed With Ron, Using First Entry Of DisplayInfo");
        println!("Display Parsed With DisplayInfo: {}", display_info.name);
        return (display_info.width, display_info.height);
    }

    (1920, 1080)
}
