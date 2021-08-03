pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<u8, i32> = HashMap::with_capacity(7);
    map.insert(b'I', 1);
    map.insert(b'V', 5);
    map.insert(b'X', 10);
    map.insert(b'L', 50);
    map.insert(b'C', 100);
    map.insert(b'D', 500);
    map.insert(b'M', 1000);

    let mut sum = 0;
    let s = s.into_bytes();
    for i in 0..s.len() - 1 {
        let left = map.get(&s[i]).unwrap();
        let right = map.get(&s[i + 1]).unwrap();
        if left < right {
            sum -= left;
        } else {
            sum += left;
        }
    }
    sum += map.get(&s[s.len() - 1]).unwrap();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("IX")), 9);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }
}
