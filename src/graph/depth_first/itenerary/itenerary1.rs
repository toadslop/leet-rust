use std::collections::{hash_map::Entry, HashMap};

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut edge_map = tickets.iter().fold(HashMap::new(), into_edgelist);
        let mut visited_map = HashMap::new();

        let mut itenerary = Vec::with_capacity(tickets.len() + 1);
        let mut indexes = Vec::with_capacity(tickets.len() + 1);

        for (&origin, dests) in edge_map.iter_mut() {
            dests.sort_by(|a, b| b.cmp(a));
            visited_map.insert(origin, vec![false; dests.len()]);
        }

        let mut stack = Vec::with_capacity(tickets.len() + 1);
        stack.push((None, "JFK", 0));

        while let Some((prev, origin, i)) = stack.pop() {
            if let Some(visited_list) = visited_map.get_mut(prev.unwrap_or("")) {
                visited_list[i] = true;
            }
            itenerary.push(origin.to_owned());
            indexes.push(i);

            if itenerary.len() == tickets.len() + 1 {
                break;
            }

            if !edge_map.contains_key(origin) || !visited_map.get(origin).unwrap().contains(&false)
            {
                loop {
                    if let Some((prev_prev, _, _)) = stack.last() {
                        if itenerary.last().unwrap() != prev_prev.unwrap() {
                            itenerary.pop().unwrap();
                            let unvisited_index = indexes.pop().unwrap();
                            visited_map
                                .get_mut(itenerary.last().unwrap().as_str())
                                .unwrap()[unvisited_index] = false;
                        } else {
                            break;
                        }
                    }
                }
                continue;
            }

            let visited_list = visited_map.get_mut(origin).unwrap();

            for (j, &dest) in edge_map.get(origin).unwrap().iter().enumerate() {
                if !visited_list[j] {
                    stack.push((Some(origin), dest, j));
                }
            }
        }

        itenerary
    }
}

fn into_edgelist<'a>(
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

    #[test]
    fn example_3() -> () {
        let tickets = [["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]]
            .iter()
            .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
            .collect();

        let output: Vec<String> = vec!["JFK", "NRT", "JFK", "KUL"]
            .iter()
            .map(|&str| String::from(str))
            .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }

    #[test]
    fn example_4() -> () {
        let tickets = [
            ["EZE", "AXA"],
            ["TIA", "ANU"],
            ["ANU", "JFK"],
            ["JFK", "ANU"],
            ["ANU", "EZE"],
            ["TIA", "ANU"],
            ["AXA", "TIA"],
            ["TIA", "JFK"],
            ["ANU", "TIA"],
            ["JFK", "TIA"],
        ]
        .iter()
        .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
        .collect();

        let output: Vec<String> = vec![
            "JFK", "ANU", "EZE", "AXA", "TIA", "ANU", "JFK", "TIA", "ANU", "TIA", "JFK",
        ]
        .iter()
        .map(|&str| String::from(str))
        .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }

    #[test]
    fn example_5() -> () {
        let tickets = [
            ["EZE", "TIA"],
            ["EZE", "HBA"],
            ["AXA", "TIA"],
            ["JFK", "AXA"],
            ["ANU", "JFK"],
            ["ADL", "ANU"],
            ["TIA", "AUA"],
            ["ANU", "AUA"],
            ["ADL", "EZE"],
            ["ADL", "EZE"],
            ["EZE", "ADL"],
            ["AXA", "EZE"],
            ["AUA", "AXA"],
            ["JFK", "AXA"],
            ["AXA", "AUA"],
            ["AUA", "ADL"],
            ["ANU", "EZE"],
            ["TIA", "ADL"],
            ["EZE", "ANU"],
            ["AUA", "ANU"],
        ]
        .iter()
        .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
        .collect();

        let output: Vec<String> = vec![
            "JFK", "AXA", "AUA", "ADL", "ANU", "AUA", "ANU", "EZE", "ADL", "EZE", "ANU", "JFK",
            "AXA", "EZE", "TIA", "AUA", "AXA", "TIA", "ADL", "EZE", "HBA",
        ]
        .iter()
        .map(|&str| String::from(str))
        .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }

    #[test]
    fn example_6() -> () {
        let tickets = [
            ["AUA", "PER"],
            ["LST", "ADL"],
            ["CNS", "TIA"],
            ["ADL", "VIE"],
            ["ADL", "VIE"],
            ["BNE", "CBR"],
            ["EZE", "VIE"],
            ["JFK", "ADL"],
            ["CBR", "HBA"],
            ["CNS", "AUA"],
            ["HBA", "BNE"],
            ["OOL", "LST"],
            ["PER", "AUA"],
            ["SYD", "AXA"],
            ["TIA", "BNE"],
            ["MEL", "AXA"],
            ["AUA", "OOL"],
            ["LST", "OOL"],
            ["DRW", "SYD"],
            ["CNS", "SYD"],
            ["INN", "CBR"],
            ["BNE", "INN"],
            ["BNE", "EZE"],
            ["BNE", "CNS"],
            ["OOL", "DRW"],
            ["BNE", "EZE"],
            ["CBR", "BNE"],
            ["TIA", "LST"],
            ["OOL", "JFK"],
            ["SYD", "CBR"],
            ["PER", "MEL"],
            ["HBA", "OOL"],
            ["MEL", "EZE"],
            ["OOL", "HBA"],
            ["AUA", "PER"],
            ["DRW", "HBA"],
            ["VIE", "ANU"],
            ["HBA", "BNE"],
            ["DRW", "TIA"],
            ["AXA", "VIE"],
            ["LST", "BNE"],
            ["CNS", "MEL"],
            ["ADL", "HBA"],
            ["VIE", "OOL"],
            ["TIA", "MEL"],
            ["PER", "DRW"],
            ["INN", "CNS"],
            ["JFK", "LST"],
            ["LST", "DRW"],
            ["MEL", "TIA"],
            ["EZE", "CNS"],
            ["AXA", "CNS"],
            ["ADL", "LST"],
            ["TIA", "JFK"],
            ["VIE", "SYD"],
            ["INN", "JFK"],
            ["VIE", "ADL"],
            ["SYD", "AUA"],
            ["ANU", "INN"],
            ["BNE", "SYD"],
            ["JFK", "INN"],
            ["SYD", "PER"],
            ["ADL", "TIA"],
            ["JFK", "ADL"],
            ["CBR", "ADL"],
            ["EZE", "BNE"],
        ]
        .iter()
        .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
        .collect();

        let output: Vec<String> = vec![
            "JFK", "ADL", "HBA", "BNE", "CBR", "ADL", "LST", "ADL", "TIA", "BNE", "CNS", "AUA",
            "OOL", "DRW", "HBA", "BNE", "EZE", "BNE", "EZE", "CNS", "MEL", "AXA", "CNS", "SYD",
            "AUA", "PER", "AUA", "PER", "DRW", "SYD", "AXA", "VIE", "ADL", "VIE", "ANU", "INN",
            "CBR", "BNE", "INN", "CNS", "TIA", "JFK", "ADL", "VIE", "OOL", "HBA", "OOL", "JFK",
            "INN", "JFK", "LST", "BNE", "SYD", "PER", "MEL", "TIA", "LST", "OOL", "LST", "DRW",
            "TIA", "MEL", "EZE", "VIE", "SYD", "CBR", "HBA",
        ]
        .iter()
        .map(|&str| String::from(str))
        .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }

    #[test]
    fn example_7() -> () {
        let tickets = [
            ["DRW", "OOL"],
            ["OOL", "ANU"],
            ["OOL", "INN"],
            ["EZE", "ELH"],
            ["ADL", "FPO"],
            ["INN", "AUA"],
            ["FPO", "MEL"],
            ["AUA", "LST"],
            ["HBA", "CNS"],
            ["CNS", "MEL"],
            ["ASD", "LST"],
            ["SYD", "MEL"],
            ["OOL", "FPO"],
            ["AUA", "ADL"],
            ["CNS", "BIM"],
            ["BAK", "ELH"],
            ["ELH", "MEL"],
            ["DRW", "PER"],
            ["BIM", "EZE"],
            ["AXA", "ANU"],
            ["VIE", "MHH"],
            ["DRW", "FPO"],
            ["JFK", "OOL"],
            ["AXA", "TCB"],
            ["ANU", "AUA"],
            ["TCB", "HBA"],
            ["MEL", "SYD"],
            ["HBA", "ASD"],
            ["LST", "JFK"],
            ["HBA", "TIA"],
            ["TCB", "BAK"],
            ["CBR", "TBI"],
            ["DRW", "ASD"],
            ["ELH", "BAK"],
            ["BAK", "SYD"],
            ["LST", "AXA"],
            ["AXA", "CNS"],
            ["MEL", "BAK"],
            ["HBA", "BAK"],
            ["ANU", "AXA"],
            ["FPO", "HBA"],
            ["OOL", "BIM"],
            ["ASD", "ELH"],
            ["ELH", "DRW"],
            ["FPO", "MEL"],
            ["MEL", "CBR"],
            ["AUA", "DRW"],
            ["EZE", "AXA"],
            ["MHH", "BNE"],
            ["ANU", "BAK"],
            ["BAK", "BNE"],
            ["MEL", "DRW"],
            ["TBI", "EZE"],
            ["CBR", "ELH"],
            ["OOL", "AUA"],
            ["ADL", "HBA"],
            ["TCB", "OOL"],
            ["CBR", "MEL"],
            ["LST", "ANU"],
            ["BAK", "TBI"],
            ["AUA", "ANU"],
            ["HBA", "AXA"],
            ["TIA", "DRW"],
            ["BNE", "TIA"],
            ["ADL", "CBR"],
            ["TBI", "TCB"],
            ["TIA", "BNE"],
            ["SYD", "CBR"],
            ["ELH", "HBA"],
            ["AXA", "DRW"],
            ["MEL", "SYD"],
            ["TBI", "AUA"],
            ["CNS", "TBI"],
            ["FPO", "LST"],
            ["SYD", "LST"],
            ["ELH", "VIE"],
            ["BIM", "CNS"],
            ["JFK", "TIA"],
            ["PER", "ADL"],
            ["BAK", "ADL"],
            ["BNE", "TCB"],
            ["EZE", "SYD"],
            ["BNE", "OOL"],
            ["DRW", "FPO"],
            ["LST", "DRW"],
            ["SYD", "EZE"],
            ["MEL", "OOL"],
            ["TIA", "HBA"],
            ["ANU", "ELH"],
        ]
        .iter()
        .map(|&edge| edge.iter().map(|&str| String::from(str)).collect())
        .collect();

        let expected: Vec<String> = vec![
            "JFK", "OOL", "ANU", "AUA", "ADL", "CBR", "ELH", "BAK", "ADL", "FPO", "HBA", "ASD",
            "ELH", "DRW", "ASD", "LST", "ANU", "AXA", "ANU", "BAK", "BNE", "OOL", "AUA", "ANU",
            "ELH", "HBA", "AXA", "CNS", "BIM", "CNS", "MEL", "BAK", "ELH", "MEL", "CBR", "MEL",
            "DRW", "FPO", "LST", "AXA", "DRW", "FPO", "MEL", "OOL", "BIM", "EZE", "AXA", "TCB",
            "BAK", "SYD", "CBR", "TBI", "AUA", "DRW", "OOL", "FPO", "MEL", "SYD", "EZE", "ELH",
            "VIE", "MHH", "BNE", "TCB", "HBA", "BAK", "TBI", "EZE", "SYD", "MEL", "SYD", "LST",
            "DRW", "PER", "ADL", "HBA", "CNS", "TBI", "TCB", "OOL", "INN", "AUA", "LST", "JFK",
            "TIA", "BNE", "TIA", "HBA", "TIA", "DRW",
        ]
        .iter()
        .map(|&str| String::from(str))
        .collect();

        let output = Solution::find_itinerary(tickets);

        assert_eq!(output, expected);
    }
}
// "SYD", "PER", "MEL", "TIA", "LST", "OOL", "LST", "DRW",
// "TIA", "MEL", "EZE", "VIE", "SYD", "CBR", "HBA",
