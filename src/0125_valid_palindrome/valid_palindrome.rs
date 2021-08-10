pub fn is_palindrome(s: String) -> bool {
    let iter = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    iter.clone().eq(iter.rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: s = "A man, a plan, a canal: Panama"
        // Output: true
        let s1 = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(s1), true);
        // Input: s = "race a car"
        // Output: false
        let s2 = String::from("race a car");
        assert_eq!(is_palindrome(s2), false);
    }
}
