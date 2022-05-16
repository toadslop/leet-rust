impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let words = Vec::from(["wrt", "wrf", "er", "ett", "rftt"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("wertf");
        let actual_result = Solution::alien_order(words);
    }

    #[test]
    fn example_2() {
        let words = Vec::from(["z", "x"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("zx");
        let actual_result = Solution::alien_order(words);
    }

    #[test]
    fn example_3() {
        let words = Vec::from(["z", "x", "z"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("");
        let actual_result = Solution::alien_order(words);
    }
}

struct Solution {}
