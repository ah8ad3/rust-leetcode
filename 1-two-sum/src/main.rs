use std::collections::HashMap;
use std::convert::TryInto;


fn main() {
    assert_eq!(two_sum(vec![1, 2, 3, 4], 6), vec![1, 3]);
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])

}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (index, value) in nums.into_iter().enumerate() {
        let comp = target - value;
        if map.contains_key(&comp) {
            return vec![map[&comp], index.try_into().unwrap()];
        }
        map.insert(value, index.try_into().unwrap());
    }
    vec![]
}
