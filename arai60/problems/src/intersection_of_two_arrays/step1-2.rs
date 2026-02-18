use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let intersection_set = HashSet::new();

        for n2 in nums2 {
            if nums1_contains(&n2) {
                intersection_set.insert(n2);
            }
        }

        intersection_set.into_iter().collect()::<Vec<_>>()
    }
}
