impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        
        fn can_finish(piles: &Vec<i32>, speed:i32, h :i32) -> bool {
            let mut hours = 0;
            for &pile in piles {
                hours += (pile + speed - 1) / speed;
                if hours > h {
                    return false;
                }
            }
            true
        }
        
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut result = right;
        
        
        while left <= right {
            let mid = left + (right-left)/2;
            if can_finish(&piles, mid, h) {
              result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        result
        
    }
}