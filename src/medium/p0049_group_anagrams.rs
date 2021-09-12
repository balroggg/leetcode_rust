use std::collections::HashMap;

#[allow(clippy::or_fun_call)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let groups: HashMap<String, Vec<String>> = strs.iter().fold(HashMap::new(), |mut acc, s| {
        let mut word: Vec<char> = s.clone().chars().collect();
        word.sort_unstable();
        let word_sorted = word.iter().collect::<String>();
        acc.entry(word_sorted)
            .or_insert(Vec::new())
            .push(s.to_string());
        acc
    });

    groups.values().fold(Vec::new(), |mut anagrams, v| {
        anagrams.push(v.to_vec());
        anagrams
    })
}

#[allow(clippy::or_fun_call)]
pub fn group_anagrams_v2(strs: Vec<String>) -> Vec<Vec<String>> {
    strs.into_iter()
        .map(|s| {
            let mut b = s.as_bytes().to_vec();
            b.sort_unstable();
            (b.to_vec(), s)
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            acc.entry(key).or_insert(Vec::new()).push(val);
            acc
        })
        .into_iter()
        .map(|(_key, val)| val)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: strs = ["eat","tea","tan","ate","nat","bat"]
        // Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
        let strs = [
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ]
        .to_vec();
        let exp = [
            [String::from("bat")].to_vec(),
            [String::from("nat"), String::from("tan")].to_vec(),
            [
                String::from("ate"),
                String::from("eat"),
                String::from("tea"),
            ]
            .to_vec(),
        ]
        .to_vec();

        // helper closure to compare test results in any order
        let to_vec_strings = |v: Vec<Vec<String>>| -> Vec<String> {
            v.iter().fold(Vec::new(), |mut acc, x| {
                for v in x {
                    acc.push(v.clone());
                }
                acc
            })
        };
        let mut output = to_vec_strings(group_anagrams(strs.clone()));
        output.sort_unstable();
        let mut exp = to_vec_strings(exp);
        exp.sort_unstable();
        // check group_anagrams
        assert_eq!(output, exp);

        let mut output = to_vec_strings(group_anagrams_v2(strs.clone()));
        output.sort_unstable();
        // check group_anagrams_v2
        assert_eq!(output, exp);

        // Input: strs = [""]
        // Output: [[""]]
        let strs = [String::from("")].to_vec();
        let exp = [[String::from("")].to_vec()].to_vec();
        assert_eq!(group_anagrams(strs), exp);

        // Input: strs = ["a"]
        // Output: [["a"]]
        let strs = [String::from("a")].to_vec();
        let exp = [[String::from("a")].to_vec()].to_vec();
        assert_eq!(group_anagrams(strs), exp);
    }
}
