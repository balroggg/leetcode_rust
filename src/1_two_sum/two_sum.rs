use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, x) in nums.iter().enumerate() {
        let complement = target - x;
        if let Some(&number) = map.get(&complement) {
            return vec![number, i as i32];
        }
        map.insert(x, i as i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(two_sum([2, 7, 11, 15].to_vec(), 9), [0, 1].to_vec());
        assert_eq!(two_sum([3, 2, 4].to_vec(), 6), [1, 2].to_vec());
        assert_eq!(two_sum([3, 3].to_vec(), 6), [0, 1].to_vec());
    }
}
