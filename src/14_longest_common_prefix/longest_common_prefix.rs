pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    if strs.is_empty() {
        return prefix;
    } else if strs.len() == 1 {
        return strs[0].clone();
    }

    let mut k: usize = 0;
    loop {
        let current_char: char = match strs[0].chars().nth(k) {
            None => {
                return prefix;
            }
            Some(c) => c,
        };
        for s in strs.iter().skip(1) {
            match s.chars().nth(k) {
                Some(c) if current_char == c => {}
                _ => return prefix,
            }
        }

        prefix.push(current_char);
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
