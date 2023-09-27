fn main() {
    assert_eq!(max_area_v1(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(max_area_v1(vec![1,1]), 1);

    assert_eq!(max_area_v2(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(max_area_v2(vec![1,1]), 1);
}


// this answer has issues with time limit
fn max_area_v1(height: Vec<i32>) -> i32 {
    let mut max_area: i32 = 0;
    let mean = (height.iter().sum::<i32>() as f32 / height.len() as f32) as i32;

    for (index, value) in height.iter().enumerate() {
        for (mut index2, value2) in height[index..].iter().enumerate() {
            if value2 < &mean {
                continue;
            }
            index2 += index;
            let min_value = std::cmp::min(value, value2);
            let area = min_value*(index2 as i32-index as i32);
            if area > max_area{
                max_area = area
            }
        }
    }
    max_area
}

fn max_area_v2(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len()-1;
    let mut max_area = 0;

    while left < right {
        let current_area = std::cmp::min(height[left], height[right]) * (right-left)as i32;
        max_area = std::cmp::max(max_area, current_area);

        if height[left] > height[right] {
            right -= 1
        }else {
            left += 1
        }
    }
    max_area
}

