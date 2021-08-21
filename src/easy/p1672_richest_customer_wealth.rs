pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|x| x.iter().sum()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: accounts = [[1,2,3],[3,2,1]]
        // Output: 6
        assert_eq!(maximum_wealth([[1,2,3].to_vec(),[3,2,1].to_vec()].to_vec()), 6);
        
        // Input: accounts = [[1,5],[7,3],[3,5]]
        // Output: 10
        assert_eq!(maximum_wealth([[1,2,3].to_vec(),[3,2,1].to_vec()].to_vec()), 6);
        
        // Input: accounts = [[2,8,7],[7,1,3],[1,9,5]]
        // Output: 17
        assert_eq!(maximum_wealth([[1,2,3].to_vec(), [3,2,1].to_vec()].to_vec()), 6);
    }
}
