use std::collections::{hash_map::Entry, HashMap};

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut edge_list: HashMap<&str, Vec<&str>> =
            tickets.iter().fold(HashMap::new(), build_edgelist);
        edge_list
            .values_mut()
            .map(|edges| {
                edges.sort_by(|a, b| b.cmp(a));
                edges
            })
            .collect();
        println!("{:?}", edge_list);
        unimplemented!()
    }
}

fn build_edgelist<'a>(
    mut map: HashMap<&'a str, Vec<&'a str>>,
    edge: &'a Vec<String>,
) -> HashMap<&'a str, Vec<&'a str>> {
    match map.entry(&edge[0]) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(&edge[1]);
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![&edge[1]]);
        }
    };
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let tickets = [
            ["MUC", "LHR"],
            ["JFK", "MUC"],
            ["SFO", "SJC"],
            ["LHR", "SFO"],
        ]
        .iter()
        .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
        .collect();

        let output: Vec<String> = vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
            .iter()
            .map(|&str| String::from(str))
            .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }

    #[test]
    fn example_2() -> () {
        let tickets = [
            ["JFK", "SFO"],
            ["JFK", "ATL"],
            ["SFO", "ATL"],
            ["ATL", "JFK"],
            ["ATL", "SFO"],
        ]
        .iter()
        .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
        .collect();

        let output: Vec<String> = vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
            .iter()
            .map(|&str| String::from(str))
            .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }
}
