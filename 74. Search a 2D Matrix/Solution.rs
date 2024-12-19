impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        
        let rows = matrix.len();
        if rows == 0 {
            return false;
        }
        
        let cols = matrix[0].len();
        
        let mut left = 0 ;
        let mut right = (rows * cols) as i32 -1;
        
        while left <= right {
            let mid = left + (right - left)/2;
            let mid_value = matrix[(mid/cols as i32) as usize][(mid % cols as i32)as usize];
             if mid_value == target {
                 return true;
            } else if mid_value < target {
                left = mid + 1;
            } else {
                right = mid -1;
            }
        }
        false
        
        }
        
    }
