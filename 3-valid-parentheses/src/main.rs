use std::collections::HashMap;

fn main() {
    assert_eq!(is_valid("()".to_string()), true);
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("(]".to_string()), false);
    assert_eq!(is_valid("({})".to_string()), true);
    assert_eq!(is_valid(")".to_string()), false);
    assert_eq!(is_valid("))".to_string()), false);
}

fn is_valid(s: String) -> bool {
    if s.len() == 1 {
        return false;
    }
    let mut backward_vec: Vec<&char> = vec![];
    let mut parentheses: HashMap<char, char> = HashMap::new();
    parentheses.insert('(', ')');
    parentheses.insert('[', ']');
    parentheses.insert('{', '}');
    for item in s.chars() {
        if parentheses.contains_key(&item) {
            backward_vec.push(parentheses.get(&item).unwrap());
        } else {
            if &item != backward_vec.pop().unwrap_or_else(|| &'-') {
                return false;
            }
        }
    }
    if backward_vec.len() != 0 {
        return false;
    }
    true
}
