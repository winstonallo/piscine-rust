pub mod lib {
    pub fn strpcmp(s: &[u8], p: &[u8]) -> bool {
        fn match_helper(s: &[u8], p: &[u8], s_idx: usize, p_idx: usize) -> bool {
            if p_idx == p.len() {
                return s_idx == s.len();
            }
    
            let match_char = s_idx < s.len() && (p[p_idx] == s[s_idx] || p[p_idx] == b'*');
    
            if p[p_idx] == b'*' {
                return match_helper(s, p, s_idx, p_idx + 1) || (match_char && match_helper(s, p, s_idx + 1, p_idx));
            } else {
                return match_char && match_helper(s, p, s_idx + 1, p_idx + 1);
            }
        }
        match_helper(s, p, 0, 0)
    }
}