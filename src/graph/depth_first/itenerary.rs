struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        unimplemented!()
    }
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
        .map(|&edge| vec![edge.iter().map(|&str| String::from(str)).collect()])
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
        .map(|&edge| vec![edge.iter().map(|&str| String::from(str)).collect()])
        .collect();

        let output: Vec<String> = vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
            .iter()
            .map(|&str| String::from(str))
            .collect();

        let result = Solution::find_itinerary(tickets);

        assert_eq!(result, output);
    }
}
