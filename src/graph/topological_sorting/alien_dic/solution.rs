use std::collections::{HashMap, VecDeque};

use self::utils::populate_counts_and_opts;

#[allow(dead_code)]
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut prev_counts = HashMap::new();
        let mut next_options = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = String::new();
        let invalid = String::from("");

        if populate_counts_and_opts(words, &mut next_options, &mut prev_counts) {
            for (char, count) in prev_counts.iter() {
                if *count == 0 {
                    queue.push_back(*char);
                }
            }

            while let Some(char) = queue.pop_front() {
                result.push(char);

                for next_char in next_options.get(&char).unwrap().iter() {
                    *prev_counts.get_mut(next_char).unwrap() -= 1;
                    if *prev_counts.get(next_char).unwrap() == 0 {
                        queue.push_back(*next_char);
                    }
                }
            }

            if result.len() != next_options.keys().len() {
                invalid
            } else {
                result
            }
        } else {
            invalid
        }
    }
}

#[allow(dead_code)]
mod utils {
    use std::collections::{HashMap, HashSet};

    pub fn populate_counts_and_opts(
        words: Vec<String>,
        next_options: &mut HashMap<char, HashSet<char>>,
        prev_counts: &mut HashMap<char, i8>,
    ) -> bool {
        for (i, word1) in words.iter().enumerate() {
            let mut found_diff = false;

            for (j, char) in word1.chars().enumerate() {
                prev_counts.entry(char).or_insert(0);
                next_options.entry(char).or_insert(HashSet::new());

                if i < words.len() - 1 {
                    if let Some(next_char) = words[i + 1].chars().nth(j) {
                        if next_char != char && !found_diff {
                            found_diff = true;
                            let opts = next_options.entry(char).or_insert(HashSet::new());

                            if !opts.contains(&next_char) {
                                opts.insert(next_char);
                                *prev_counts.entry(next_char).or_insert(0) += 1;
                            }
                        }
                    } else if !found_diff {
                        return false;
                    }
                }
            }
        }

        true
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

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_2() {
        let words = Vec::from(["z", "x"])
            .iter()
            .map(|&word| String::from(word))
            .collect();

        let expected_result = String::from("zx");
        let actual_result = Solution::alien_order(words);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_3() {
        let words = Vec::from(["z", "x", "z"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("");
        let actual_result = Solution::alien_order(words);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_4() {
        let words = Vec::from(["z", "z"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("z");
        let actual_result = Solution::alien_order(words);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_5() {
        let words = Vec::from(["ab", "adc"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_results: Vec<String> = Vec::from(["abcd", "bcad", "acbd", "bacd", "cbad"])
            .iter()
            .map(|&word| String::from(word))
            .collect();

        let actual_result = Solution::alien_order(words);
        println!("{}", actual_result);
        assert!(expected_results.contains(&actual_result))
    }

    #[test]
    fn example_6() {
        let words = Vec::from(["ac", "ab", "zc", "zb"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_results: Vec<String> = Vec::from(["acbz", "acbz", "cabz", "aczb"])
            .iter()
            .map(|&word| String::from(word))
            .collect();

        let actual_result = Solution::alien_order(words);

        println!("{}", actual_result);
        assert!(expected_results.contains(&actual_result))
    }

    #[test]
    fn example_7() {
        let words = Vec::from(["abc", "ab"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("");
        let actual_result = Solution::alien_order(words);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_8() {
        let words = Vec::from(["z", "x", "a", "zb", "zx"])
            .iter()
            .map(|&word| String::from(word))
            .collect();
        let expected_result = String::from("");
        let actual_result = Solution::alien_order(words);

        assert_eq!(expected_result, actual_result)
    }
}

#[allow(dead_code)]
struct Solution {}
