struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let equations = Vec::from([["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let values = vec![2.0, 3.0];
        let queries = Vec::from([["a", "b"], ["b", "c"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let result = Solution::calc_equation(equations, values, queries);

        assert_eq!(result, [6.00000, 0.50000, -1.00000, 1.00000, -1.00000]);
    }

    #[test]
    fn example_2() -> () {
        let equations = Vec::from([["a", "b"], ["b", "c"], ["bc", "cd"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let values = vec![1.5, 2.5, 5.0];
        let queries = Vec::from([["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let result = Solution::calc_equation(equations, values, queries);

        assert_eq!(result, [3.75000, 0.40000, 5.00000, 0.20000]);
    }

    #[test]
    fn example_3() -> () {
        let equations = Vec::from([["a", "b"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let values = vec![0.5];
        let queries = Vec::from([["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let result = Solution::calc_equation(equations, values, queries);

        assert_eq!(result, [0.50000, 2.00000, -1.00000, -1.00000]);
    }
}
