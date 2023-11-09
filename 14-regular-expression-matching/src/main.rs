fn main() {
    assert_eq!(is_match("aa".to_string(), "aaa".to_string()), false);
    assert_eq!(is_match("aa".to_string(), "aa".to_string()), true);
    assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(is_match("aabbc".to_string(), "a*bbc".to_string()), true);
    assert_eq!(is_match("ab".to_string(), ".*".to_string()), true);
    assert_eq!(is_match("aab".to_string(), "c*a*b".to_string()), true);
    assert_eq!(is_match("mississippi".to_string(), "mis*is*ip*.".to_string()), true);
    assert_eq!(is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    assert_eq!(is_match("aaaajklbdfqwe".to_string(), "a*...bc*df...".to_string()), true);
    assert_eq!(is_match("ab".to_string(), ".*c".to_string()), false);
    assert_eq!(is_match("aaa".to_string(), "aa*a".to_string()), true);
    assert_eq!(is_match("ababaa".to_string(), "aa*baa*baa*a".to_string()), true);
    assert_eq!(is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
    assert_eq!(is_match("bbbba".to_string(), ".*a*a".to_string()), true);
    assert_eq!(is_match("a".to_string(), ".*..a*".to_string()), false);
    assert_eq!(is_match("aaaaaaaa".to_string(), ".*..a*".to_string()), true);
    assert_eq!(is_match("ab".to_string(), ".*..".to_string()), true);
    assert_eq!(is_match("ab".to_string(), ".*ab".to_string()), true);
    assert_eq!(is_match("abbbcd".to_string(), "ab*bbbcd".to_string()), true);
    assert_eq!(is_match("a".to_string(), "ab*a".to_string()), false);
    assert_eq!(is_match("a".to_string(), ".*..".to_string()), false);
    assert_eq!(is_match("aabcbcbcaccbcaabc".to_string(), ".*a*aa*.*b*.c*.*a*".to_string()), true);
}

pub fn is_match(s: String, p: String) -> bool {
    fn is_match_str(s: &str, p: &str) -> bool {
        let (s_len, p_len) = (s.len(), p.len());
        if p_len == 0 {
            return s_len == 0;
        }
        // 46 for .
        let m = { s_len > 0 && (s.as_bytes()[0] == p.as_bytes()[0] || p.as_bytes()[0] == 46) };
        // 42 for *
        if p_len >= 2 && p.as_bytes()[1] == 42 {
            return is_match_str(s, &p[2..]) || (m && is_match_str(&s[1..], p))
        }

        m && is_match_str(&s[1..], &p[1..])
    }
    is_match_str(&s, &p)
}
