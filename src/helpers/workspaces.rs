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





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
 
    // ---- no persistent workspaces ------------------------------------------
 
    #[test]
    fn real_workspaces_no_persistent_returns_sorted_real()
    {
        let result = build_workspace_list(&[3, 1, 2], None);
        assert_eq!(result, vec![1, 2, 3]);
    }
 
    #[test]
    fn empty_real_no_persistent_returns_empty()
    {
        let result = build_workspace_list(&[], None);
        assert_eq!(result, Vec::<i32>::new());
    }
 
    #[test]
    fn single_real_workspace_no_persistent()
    {
        let result = build_workspace_list(&[5], None);
        assert_eq!(result, vec![5]);
    }
 
    // ---- persistent workspaces, no real ------------------------------------
 
    #[test]
    fn persistent_only_generates_range_1_to_n()
    {
        let result = build_workspace_list(&[], Some(5));
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
 
    #[test]
    fn persistent_1_generates_single_entry()
    {
        let result = build_workspace_list(&[], Some(1));
        assert_eq!(result, vec![1]);
    }
 
    // ---- persistent + real workspaces merge --------------------------------
 
    #[test]
    fn real_within_persistent_range_not_duplicated()
    {
        // persistent = 3 → [1,2,3], real = [2,3] → no duplicates
        let result = build_workspace_list(&[2, 3], Some(3));
        assert_eq!(result, vec![1, 2, 3]);
    }
 
    #[test]
    fn real_outside_persistent_range_appended_and_sorted()
    {
        // persistent = 3 → [1,2,3], real = [5] → [1,2,3,5]
        let result = build_workspace_list(&[5], Some(3));
        assert_eq!(result, vec![1, 2, 3, 5]);
    }
 
    #[test]
    fn real_below_persistent_range_still_included()
    {
        // real workspace 1 is already covered by persistent, no duplicate
        let result = build_workspace_list(&[1, 7], Some(5));
        assert_eq!(result, vec![1, 2, 3, 4, 5, 7]);
    }
 
    #[test]
    fn result_is_always_sorted()
    {
        let result = build_workspace_list(&[9, 4, 2], Some(3));
        let mut expected = result.clone();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }
 
    #[test]
    fn real_workspaces_already_sorted_still_correct()
    {
        let result = build_workspace_list(&[1, 2, 3], None);
        assert_eq!(result, vec![1, 2, 3]);
    }
 
    #[test]
    fn duplicate_real_workspaces_deduplicated_against_persistent()
    {
        // If real contains a workspace already in persistent, no duplicate
        let result = build_workspace_list(&[1, 1, 2], Some(2));
        assert_eq!(result, vec![1, 2]);
    }
 
    #[test]
    fn large_persistent_with_extra_real_workspaces()
    {
        let result = build_workspace_list(&[11, 12], Some(10));
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }
}
