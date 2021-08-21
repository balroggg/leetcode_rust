pub fn reverse(x: i32) -> i32 {
    let mut rev = 0;
    let mut number = x;
    const MAX10: i32 = i32::MAX / 10;
    const MIN10: i32 = i32::MIN / 10;
    while number != 0 {
        let pop = number % 10;
        number /= 10;
        if rev > MAX10 || (rev == MAX10 && pop > 7) {
            return 0;
        }
        if rev < MIN10 || (rev == MIN10 && pop < -8) {
            return 0;
        }
        rev = rev * 10 + pop;
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        use std::i32;
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(i32::max_value()), 0);
        assert_eq!(reverse(i32::min_value()), 0);
    }
}
