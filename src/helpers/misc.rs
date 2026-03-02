// ============ CRATES ============
use crate::ron::{BarConfig, BarPosition};
use crate::modules::data::Modules;





// ============ FUNCTIONS ============
pub fn is_active_module(active_modules: &Vec<Modules>, module: Modules) -> bool
{
    for item in active_modules 
    {
        if *item == module 
        {
            return true;
        }
    }
    false
}



pub fn validade_bar_size_and_margin(ron_config: &BarConfig) -> (u32, (i32, i32, i32, i32))
{
    match ron_config.bar_position 
    {
        BarPosition::Up => 
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On Top!!!") }
            (ron_config.bar_size[1] + ron_config.increased_exclusive_bar_zone, (ron_config.floating_space, 0, 0 ,0))
        },
        BarPosition::Right =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Right!!!") }
            (ron_config.bar_size[0] + ron_config.increased_exclusive_bar_zone, (0, ron_config.floating_space, 0, 0))
        }
        BarPosition::Down =>
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On The Bottom!!!") }
            (ron_config.bar_size[1] + ron_config.increased_exclusive_bar_zone, (0, 0, ron_config.floating_space, 0))
        }
        BarPosition::Left =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Left!!!") }
            (ron_config.bar_size[0] + ron_config.increased_exclusive_bar_zone, (0, 0, 0, ron_config.floating_space))
        }
    }
}
