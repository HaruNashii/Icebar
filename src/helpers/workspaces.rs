// ============ FUNCTIONS ============
pub fn build_workspace_list(real: &[i32], persistent: Option<u8>) -> Vec<i32>
{
    let mut result = Vec::new();

    if let Some(max) = persistent
    {
        for id in 1..=max { result.push(id as i32); }
    }

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

    #[test]
    fn duplicate_real_workspaces_no_persistent_deduplicated()
    {
        let result = build_workspace_list(&[1, 1, 2], None);
        assert_eq!(result, vec![1, 2]);
    }

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

    #[test]
    fn real_within_persistent_range_not_duplicated()
    {
        let result = build_workspace_list(&[2, 3], Some(3));
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn real_outside_persistent_range_appended_and_sorted()
    {
        let result = build_workspace_list(&[5], Some(3));
        assert_eq!(result, vec![1, 2, 3, 5]);
    }

    #[test]
    fn real_below_persistent_range_still_included()
    {
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
        let result = build_workspace_list(&[1, 1, 2], Some(2));
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn large_persistent_with_extra_real_workspaces()
    {
        let result = build_workspace_list(&[11, 12], Some(10));
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn persistent_255_generates_all_255_entries()
    {
        let result = build_workspace_list(&[], Some(255));
        assert_eq!(result.len(), 255);
        assert_eq!(result[0], 1);
        assert_eq!(result[254], 255);
    }

    #[test]
    fn negative_real_workspaces_are_included_and_sorted()
    {
        let result = build_workspace_list(&[-2, -1, 0, 1], None);
        assert_eq!(result, vec![-2, -1, 0, 1]);
    }

    #[test]
    fn persistent_0_generates_empty_range()
    {
        let result = build_workspace_list(&[], Some(0));
        assert!(result.is_empty());
    }

    #[test]
    fn real_workspaces_with_zero_workspace_id_included()
    {
        let result = build_workspace_list(&[0, 1, 2], None);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn mixed_negative_and_positive_real_workspaces_sorted()
    {
        let result = build_workspace_list(&[3, -1, 0], None);
        assert_eq!(result, vec![-1, 0, 3]);
    }

    #[test]
    fn very_large_workspace_id_included()
    {
        let result = build_workspace_list(&[i32::MAX], None);
        assert_eq!(result, vec![i32::MAX]);
    }

    #[test]
    fn persistent_and_real_with_overlap_produces_deduplicated_sorted_list()
    {
        let result = build_workspace_list(&[1, 2, 3, 10], Some(5));
        assert_eq!(result, vec![1, 2, 3, 4, 5, 10]);
    }

    #[test]
    fn all_real_same_value_produces_single_entry()
    {
        let result = build_workspace_list(&[7, 7, 7, 7], None);
        assert_eq!(result, vec![7]);
    }

    #[test]
    fn real_workspace_id_exactly_at_persistent_boundary_not_duplicated()
    {
        let result = build_workspace_list(&[5], Some(5));
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn result_contains_no_duplicates_under_any_input()
    {
        let result = build_workspace_list(&[1, 1, 2, 2, 3], Some(3));
        let mut seen = std::collections::HashSet::new();
        for id in &result { assert!(seen.insert(id), "duplicate: {}", id); }
    }
}
