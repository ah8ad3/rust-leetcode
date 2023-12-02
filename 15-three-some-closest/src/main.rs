fn main() {
    // The sum that is closest to the target is 2. (-1 + 2 + 1 = 2)
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    // The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
    assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
}

// a + b + c - target ->(close) 0
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut best = 999999;
    let mut best_sum = 999999;
    for (index, value) in nums[..=nums.len()-2].into_iter().enumerate() {
        for (index2, value2) in nums[index+1..=nums.len()-1].into_iter().enumerate() {
            for value3 in nums[index+index2+2..].into_iter() {
                let temp = value+value2+value3;
                let sub = (temp-target).abs();
                if sub == 0 {
                    return temp;
                }
                if sub < best {
                    best = sub;
                    best_sum = temp;
                }

            }
        }
    }
    best_sum
}
