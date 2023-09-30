use std::collections::HashMap;

fn main() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(379), "CCCLXXIX".to_string());
}
// middle: v, changer: I, next: X --> V: 5, I: 1, X: 10, II: 2, IX: 9, IV: 4, VI: 6
fn int_to_roman(num: i32) -> String {
    let mut identities: HashMap<u32, HashMap<String, String>> = HashMap::new();
    identities.entry(4).or_insert_with(|| {
        let mut map = HashMap::new();
        map.extend([
            ("middle".to_string(), "".to_string()),
            ("changer".to_string(), "M".to_string()),
            ("next".to_string(), "".to_string()),
            ]);
        map
    });
    identities.entry(3).or_insert_with(|| {
        let mut map = HashMap::new();
        map.extend([
            ("middle".to_string(), "D".to_string()),
            ("changer".to_string(), "C".to_string()),
            ("next".to_string(), "M".to_string()),
            ]);
        map
    });
    identities.entry(2).or_insert_with(|| {
        let mut map = HashMap::new();
        map.extend([
            ("middle".to_string(), "L".to_string()),
            ("changer".to_string(), "X".to_string()),
            ("next".to_string(), "C".to_string()),
            ]);
        map
    });
    identities.entry(1).or_insert_with(|| {
        let mut map = HashMap::new();
        map.extend([
            ("middle".to_string(), "V".to_string()),
            ("changer".to_string(), "I".to_string()),
            ("next".to_string(), "X".to_string()),
            ]);
        map
    });

    let num_string = num.to_string();
    let num_len = num_string.len();
    let mut result = String::new();
    for (index, digit) in num_string.chars().enumerate(){
        let digit_value: u32 = digit.to_digit(10).unwrap();
        let identifier = (num_len - index) as u32;
        let identity = identities.get(&identifier).unwrap();
        match digit_value {
            0 => {continue;},
            4 => {
                result += identity.get("changer").unwrap();
                result += identity.get("middle").unwrap()
            },
            5 => {
                result += identity.get("middle").unwrap()
            },
            9 => {
                result += identity.get("changer").unwrap();
                result += identity.get("next").unwrap()
            },
            mut x => {
                if x > 5 {
                    result += identity.get("middle").unwrap();
                    x -= 5
                }
                for _ in 0..x {
                    result += identity.get("changer").unwrap()
                }
            }
        }
    }
    result
}