use std::collections::HashSet;

fn main() {
    assert_eq!(contains_duplicate(vec![1,2,3,4,5]), false);
    assert_eq!(contains_duplicate(vec![1,2,3,4,1]), true);
    assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
}

// we can convert vector to a set and compare length of them
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let nums_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    nums_set.len() != nums.len()
}