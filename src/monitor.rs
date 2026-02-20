use display_info::DisplayInfo;





pub fn get_monitor_res() -> (u32, u32)
{
    let display_infos = DisplayInfo::all().unwrap();
    let mut monitor_size = (0, 0);
    for display_info in display_infos
    {
        println!("Debug info: {:?}", display_info);
        monitor_size = (display_info.width, display_info.height);
        break;
    }
    println!("Fetched Monitor Size: {:?}", monitor_size);

    monitor_size
}
