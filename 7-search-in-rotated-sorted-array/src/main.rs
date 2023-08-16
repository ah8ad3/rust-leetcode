fn main() {
    assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
    assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
    assert_eq!(search(vec![1], 0), -1);
    assert_eq!(search(vec![1,3], 3), 1);
    assert_eq!(search(vec![3,1], 3), 0);
}

fn search(mut nums: Vec<i32>, target: i32) -> i32 {
    // check the first one if its bigger go ahead
    // if its smaller check from last
    // worst case scenario our number is in the middle
    // and it will be o(n/2 + 1)
    if Some(&target) >= nums.first() {
        // if suddenly the target is smaller than the item in loop
        // this means the target doesn't exist so we should return -1
        for (index, item) in nums.iter().enumerate() {
            if target < *item {
                return -1;
            }
            if target == *item {
                return index.try_into().unwrap();
            }
        }
    } else {
        nums.reverse();
        for (index, item) in nums.iter().enumerate() {
            if target > *item {
                return -1;
            }
            if target == *item {
                return (nums.len() -  index - 1).try_into().unwrap();
            }
        }
    }
    -1
}
