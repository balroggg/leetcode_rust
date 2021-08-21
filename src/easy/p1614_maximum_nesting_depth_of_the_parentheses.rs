pub fn max_depth(s: String) -> i32 {
    s.chars()
        .filter(|&c| "()".contains(c))
        .map(|c| if c == '(' { 1 } else { -1 })
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: s = "(1+(2*3)+((8)/4))+1"
        // Output: 3
        assert_eq!(max_depth(String::from("(1+(2*3)+((8)/4))+1")), 3);
        // Input: s = "(1)+((2))+(((3)))"
        // Output: 3
        assert_eq!(max_depth(String::from("(1)+((2))+(((3)))")), 3);
        // Input: s = "1+(2*3)/(2-1)"
        // Output: 1
        assert_eq!(max_depth(String::from("1+(2*3)/(2-1)")), 1);
        // Input: s = "1"
        // Output: 0
        assert_eq!(max_depth(String::from("1")), 0);
    }
}
