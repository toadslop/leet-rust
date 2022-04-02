use std::collections::HashMap;

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut rank = vec![0; s.len()];
        let mut root = (0..s.len()).collect::<Vec<usize>>();

        for pair in pairs.iter() {
            union(&mut root, &mut rank, pair[0] as usize, pair[1] as usize);
        }

        let sets: HashMap<usize, (Vec<char>, Vec<usize>)> = get_sorted_sets(&mut root, &s);

        let mut result = vec![' '; s.len()];

        for (chars, indexes) in sets.values() {
            for (i, char) in chars.iter().enumerate() {
                result[indexes[i]] = *char;
            }
        }

        result.iter().collect()
    }
}

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    match x == root[x] {
        true => x,
        false => {
            root[x] = find(root, root[x]);
            root[x]
        }
    }
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) -> () {
    let root_x = find(root, x);
    let root_y = find(root, y);

    if rank[root_y] > rank[root_x] {
        root[root_x] = root_y;
    } else {
        root[root_y] = root_x;
        if rank[root_y] == rank[root_x] {
            rank[root_y] += 1;
        }
    }
}

fn get_sorted_sets(mut root: &mut Vec<usize>, s: &str) -> HashMap<usize, (Vec<char>, Vec<usize>)> {
    let mut sets: HashMap<usize, (Vec<char>, Vec<usize>)> = HashMap::with_capacity(s.len());

    for (i, char) in s.chars().enumerate() {
        if sets.contains_key(&find(&mut root, i)) {
            sets.entry(root[i]).and_modify(|(chars, indexes)| {
                chars.push(char);
                indexes.push(i);
            });
        } else {
            sets.insert(root[i], (vec![char], vec![i]));
        }
    }

    for (chars, _) in sets.values_mut() {
        chars.sort();
    }

    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let s = String::from("dcab");
        let pairs = Vec::from([[0, 3], [1, 2]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::smallest_string_with_swaps(s, pairs);

        assert_eq!(result, "bacd");
    }

    #[test]
    fn example_2() -> () {
        let s = String::from("dcab");
        let pairs = Vec::from([[0, 3], [1, 2], [0, 2]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::smallest_string_with_swaps(s, pairs);

        assert_eq!(result, "abcd");
    }

    #[test]
    fn example_3() -> () {
        let s = String::from("cba");
        let pairs = Vec::from([[0, 1], [1, 2]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::smallest_string_with_swaps(s, pairs);

        assert_eq!(result, "abc");
    }
}
