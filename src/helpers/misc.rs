// ============ FUNCTIONS ============
pub fn is_active_module(active_modules: &Vec<String>, module: String) -> bool
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
