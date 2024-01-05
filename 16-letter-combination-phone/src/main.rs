use std::collections::VecDeque;

fn main() {
    assert_eq!(letter_combinations(""), Vec::new() as Vec<String>);
    assert_eq!(letter_combinations("23"), vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    assert_eq!(letter_combinations("2"), vec!["a", "b", "c"])
}

fn letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let digit_to_letters: Vec<&str> = vec![
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
    ];

    let mut result: Vec<String> = Vec::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(String::new());

    for digit in digits.chars() {
        let letters = digit_to_letters[digit.to_digit(10).unwrap() as usize];
        let queue_len = queue.len();

        for _ in 0..queue_len {
            let current_combination = queue.pop_front().unwrap();

            for letter in letters.chars() {
                let mut new_combination = current_combination.clone();
                new_combination.push(letter);
                queue.push_back(new_combination);
            }
        }
    }

    result.extend(queue.into_iter());
    result
}
