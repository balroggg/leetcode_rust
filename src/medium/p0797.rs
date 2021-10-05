// 797. All Paths From Source to Target
// https://leetcode.com/problems/all-paths-from-source-to-target/

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path: Vec<i32> = vec![];
    //let n = graph.len();
    dfs(0, &mut path, &mut res, &graph);
    fn dfs(u: i32, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>, graph: &[Vec<i32>]) {
        path.push(u);
        if u as usize == graph.len() - 1 {
            paths.push(path.clone());
        } else {
            for &v in &graph[u as usize] {
                dfs(v, path, paths, graph);
            }
        }
        path.pop();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Input: graph = [[1,2],[3],[3],[]]
        // Output: [[0,1,3],[0,2,3]]
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let res = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(all_paths_source_target(graph), res);

        // Input: graph = [[4,3,1],[3,2,4],[3],[4],[]]
        // Output: [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let res = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(all_paths_source_target(graph), res);

        // Input: graph = [[1],[]]
        // Output: [[0,1]]
        let graph = vec![vec![1], vec![]];
        let res = vec![vec![0, 1]];
        assert_eq!(all_paths_source_target(graph), res);

        // Input: graph = [[1,2,3],[2],[3],[]]
        // Output: [[0,1,2,3],[0,2,3],[0,3]]
        let graph = vec![vec![1, 2, 3], vec![2], vec![3], vec![]];
        let res = vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]];
        assert_eq!(all_paths_source_target(graph), res);

        // Input: graph = [[1,3],[2],[3],[]]
        // Output: [[0,1,2,3],[0,3]]
        let graph = vec![vec![1, 3], vec![2], vec![3], vec![]];
        let res = vec![vec![0, 1, 2, 3], vec![0, 3]];
        assert_eq!(all_paths_source_target(graph), res);
    }
}
