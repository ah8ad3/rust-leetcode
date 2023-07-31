use std::collections::HashSet;

fn main() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring(" ".to_string()), 1);
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
}

fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut current_streak = 0;
    let temp = length_of_longest_substring(s[1..].to_string());
    
    let mut set: HashSet<u8> = HashSet::new();
    for item in s.bytes() {
        if !set.contains(&item) {
            current_streak += 1;
            set.insert(item);
        } else {
            break;
        }
    }
    if temp > current_streak {
        current_streak = temp;
    }

    current_streak
}
