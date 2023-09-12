fn main() {
    assert_eq!(search_matrix(vec![vec![1,3,5,7], vec![10,11,16,20], vec![23,30,34,60]], 3), true);
    assert_eq!(search_matrix(vec![vec![1,3,5,7], vec![10,11,16,20], vec![23,30,34,60]], 13), false);
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for item in matrix {
        if &target <= item.last().unwrap() {
            if item.contains(&target) {
                return true;
            }
            break;
        }
    }
    false
}