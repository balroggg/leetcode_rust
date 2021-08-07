pub fn is_valid(s: String) -> bool {
    if s.len() < 2 {
        return false;
    }
    let mut stack = Vec::new();
    for c in s.chars() {
        match stack.last() {
            None => stack.push(c),
            Some(&top) => {
                if (top == '(' && c == ')') || (top == '[' && c == ']') || (top == '{' && c == '}')
                {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
        assert_eq!(is_valid("{[]}".to_string()), true);
        assert_eq!(is_valid("(((((())))))".to_string()), true);
        assert_eq!(is_valid("()()()()".to_string()), true);
        assert_eq!(is_valid("(((((((()".to_string()), false);
        assert_eq!(is_valid("((()(())))".to_string()), true);
    }
}
