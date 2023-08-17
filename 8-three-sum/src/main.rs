fn main() {
    assert_eq!(three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2],vec![-1,0,1]]);
    assert_eq!(three_sum(vec![0,1,1]), Vec::<Vec<i32>>::new());
    assert_eq!(three_sum(vec![0,0,0]), vec![vec![0,0,0]]);
    assert_eq!(three_sum(vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6]), vec![vec![-4,-2,6],vec![-4,0,4],vec![-4,1,3],vec![-4,2,2],vec![-2,-2,4],vec![-2,0,2]]);
}

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // should compare two i32 -> a + b = -c
    // so we should create sum of all two parts till the len-1
    // and search for -c
    let mut result:Vec<Vec<i32>> = vec![];
    nums.sort();

    // optimize to pass leet code 
    let mut positive_number_index = 0;
    let mut last_pointer: i32 = -99999999;

    for (index1, pointer1) in nums.iter().enumerate() {
        // its a sorted vec and after processsing negative numbers
        // positive one are not usefull
        if pointer1 > &0 {
            break;
        }
        if *pointer1 == last_pointer {
            continue;
        }
        for (index2, pointer2) in nums[index1+1..].iter().enumerate() {
            if pointer2 > &0  && positive_number_index == 0{
                positive_number_index = index2;
            }
            let to_match = -(pointer1 + pointer2);
            let mut range = index1+index2+2;
            if positive_number_index > range {
                range = positive_number_index
            }
            for item in nums[range..].iter() {
                // beacuse it's sorted we can break the loop at this point
                if item > &to_match {
                    break;
                }
                // we got an match
                if item == &to_match {
                    result.push(vec![*pointer1, *pointer2, to_match]);
                    // pass leet code for large number of zeroes
                    if index1 == 0 {
                        if vec![*pointer1, *pointer2, to_match] == vec![0, 0, 0] {
                            return result;
                        }
                    }
                }
            }
            last_pointer = *pointer1;
        }
    
    }
    // remove duplicated results
    result.sort_unstable();
    result.dedup();
    result
}
