fn main() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(111121111), true);
    assert_eq!(is_palindrome(123321), true);
    assert_eq!(is_palindrome(1234321), true);
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    else if x == 0 {
        return true;
    } else {
        let x_vec: Vec<u32> = x.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();

        let x_len = x_vec.len();
        let mut slow_pointer = 0;
        let mut fast_pointer = 0;
        // we should find the middle of string then reverce 
        // the second half and then just compare two strings
        // to find the middle of it we use slow and fast pointers
        // the slow one increment by one the fast one goes 
        // two step at a time and we reach end when the fast one
        //  can't go any further
        for _ in 0..x_len {
            if fast_pointer + 2 >= x_len {
                // slow pointer is in mid
                // 0..slow_pointer and revecre(slow_pointer+1..) should be equal
                // maybe it can be better rather that owning we share a refrence
                let mut second_half = x_vec[slow_pointer+1..].to_owned();
                let first_half: Vec<u32>;
                if x_len % 2 == 0 {
                    first_half = x_vec[0..slow_pointer+1].to_owned();
                } else {
                    first_half = x_vec[0..slow_pointer].to_owned();
                }
                
                second_half.reverse();
                if first_half == second_half {
                    return true;
                }
            }
            slow_pointer += 1;
            fast_pointer += 2;
            
        }
        false
    }
}
