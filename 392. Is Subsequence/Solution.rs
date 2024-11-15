impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        match s.len() {
            0 => true,
            l if l > t.len() => false,
            l if l == t.len() => s == t,
            _ => {
                let (mut s_it, mut t_it) = (s.chars(), t.chars());
                let (mut s_c, mut t_c) = (s_it.next(), t_it.next());
                while s_c.is_some() && t_c.is_some() {
                    if s_c == t_c {
                        s_c = s_it.next();
                    }
                    t_c = t_it.next();
                }
                s_c.is_none()
            }
        }
    }
}