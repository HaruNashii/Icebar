// ============ FUNCTIONS ============
pub fn build_workspace_list(real: &[i32], persistent: Option<u8>) -> Vec<i32> 
{
    let mut result = Vec::new();

    // ---------- persistent range ----------
    if let Some(max) = persistent 
    {
        for id in 1..=max { result.push(id as i32); }
    }

    // ---------- add real workspaces ----------
    for id in real 
    {
        if !result.contains(id) { result.push(*id); }
    }

    result.sort_unstable();
    result
}
