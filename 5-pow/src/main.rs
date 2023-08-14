fn main() {
    assert_eq!(my_pow(2.00000, 10), 1024.00000);
    assert_eq!(my_pow(2.10000, 3), 9.26100);
    assert_eq!(my_pow(2.00000, -2), 0.25000);
    assert_eq!(my_pow(1.00000, 2147483647), 1.0);
    assert_eq!(my_pow(2.00000, -2147483648), 0.0);
    assert_eq!(my_pow(-2.00000, 2), 4.0);
    assert_eq!(my_pow(-1.00000, -2147483648), 1.0);
}

// this is not a simple i32.pow()
// this is an implentation of actual pow
fn my_pow(x: f64, n: i32) -> f64 {
    let mut result = 1.0;

    // some optimazation for passing leet code
    if n == 0 {
        return 1.0;
    }
    if x == 1.0 {
        return result;
    }
    if x == -1.0 {
        if n % 2 == 0{
            return 1.0;
        }
        return -1.0;
    }

    let mut pow = x;
    let mut range = 0..n; 
    if n < 0 {
        pow = 1.00000/x;
        range = n..0;
    }
    for _ in range {
        result *= pow;
        // because rust compute float all points
        // we need to limit this at some point to not get time exceeded
        if result < 0.000009 && result > 0.0 {
            return 0.0;
        }
    }
    (result * 1e5).trunc() / 1e5
}
