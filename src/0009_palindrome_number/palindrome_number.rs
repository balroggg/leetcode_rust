pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let (mut x, mut rev) = (x, 0);
    while x > rev {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    x == rev || x == rev / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(-101), false);
        assert_eq!(is_palindrome(1001), true);
    }
}
