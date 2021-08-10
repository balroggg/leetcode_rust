pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut n = 0;
    while n < arr.len() - 1 {
        if arr[n] == 0 {
            arr.pop();
            arr.insert(n, 0);
            n += 2;
        } else {
            n += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: arr = [1,0,2,3,0,4,5,0]
        // Output: [1,0,0,2,3,0,0,4]
        let mut arr1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut arr1);
        assert_eq!(arr1, vec![1, 0, 0, 2, 3, 0, 0, 4]);

        // Input: arr = [1,2,3]
        // Output: [1,2,3]
        let mut arr2 = vec![1, 2, 3];
        duplicate_zeros(&mut arr2);
        assert_eq!(arr2, vec![1, 2, 3]);
    }
}
