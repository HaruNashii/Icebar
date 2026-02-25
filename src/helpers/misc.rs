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
