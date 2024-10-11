impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // Trim any trailing spaces from the string
        let trimmed = s.trim();
        
        // Split the string by spaces and collect into a vector
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        
        // Get the last word and return its length, or 0 if there are no words
        if let Some(last_word) = words.last() {
            last_word.len() as i32
        } else {
            0
        }
    }
}
