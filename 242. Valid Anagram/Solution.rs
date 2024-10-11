impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false; // If lengths are different, they can't be anagrams
        }

        let mut count_s = [0; 26]; // To count character frequencies in s
        let mut count_t = [0; 26]; // To count character frequencies in t

        for (cs, ct) in s.chars().zip(t.chars()) {
            count_s[(cs as usize) - ('a' as usize)] += 1;
            count_t[(ct as usize) - ('a' as usize)] += 1;
        }

        count_s == count_t // If both frequency arrays are equal, s and t are anagrams
    }
}
