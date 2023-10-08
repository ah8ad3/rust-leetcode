fn main() {
    assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
    assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string());
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    match strs.iter().min_by_key(|&x| x.len()) {
        Some(x) => {
            let mut prefix = String::new();
            for (index, item) in x.chars().enumerate() {
                if !strs.iter().all(|x| x.chars().nth(index) == Some(item)) {
                    return prefix;
                }
                prefix.push(item);
            }
            return prefix;
        },
        None => {return "".to_string();}
    }
}
