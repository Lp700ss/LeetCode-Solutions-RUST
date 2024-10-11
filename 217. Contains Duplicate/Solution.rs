impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        
        let mut seen = HashSet::new();

        for num in nums {
            if !seen.insert(num) {
                return true; // Duplicate found
            }
        }

        false // No duplicates found
    }
}
