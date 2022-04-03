use std::collections::HashMap;

struct Solution {}

type Equation = Vec<String>;

#[allow(dead_code)]
impl Solution {
    pub fn calc_equation(
        equations: Vec<Equation>,
        values: Vec<f64>,
        queries: Vec<Equation>,
    ) -> Vec<f64> {
        let mut value_map: HashMap<&str, (&str, f64)> = HashMap::new();
        let elements = get_elements(&equations);
        for element in elements.iter() {
            value_map.insert(*element, (*element, 1 as f64));
        }

        for (i, equation) in equations.iter().enumerate() {
            union(&mut value_map, &equation[0], &equation[1], values[i])
        }
        // println!("VALUE MAP: {:?}", value_map);
        queries
            .iter()
            .map(|query| {
                if !connected(&mut value_map, &query[0], &query[1]) {
                    -1 as f64
                } else {
                    let (_, dividend) = value_map[query[0].as_str()];
                    let (_, divisor) = value_map[query[1].as_str()];
                    // println!("VALUE MAP: {:?}", value_map);
                    // println!("{} / {}", query[0], query[1]);
                    // println!("{} / {}", dividend, divisor);
                    find(&mut value_map, "x1");
                    dividend / divisor
                }
            })
            .collect()
    }
}

fn get_elements<'a>(queries: &'a Vec<Vec<String>>) -> Vec<&'a str> {
    let mut elements: Vec<&str> = queries.iter().flatten().map(String::as_str).collect();
    elements.sort();
    elements.dedup();
    elements
}

fn find<'a>(value_map: &mut HashMap<&'a str, (&'a str, f64)>, x: &'a str) -> Option<&'a str> {
    if let Some((group_id, weight)) = value_map.get(x).copied() {
        if !x.eq(group_id) {
            let (parent_group_id, parent_weight) = value_map[group_id];
            let update = find(value_map, parent_group_id).unwrap();
            value_map.insert(x, (update, weight * parent_weight));
            Some(value_map[update].0)
        } else {
            Some(value_map[x].0)
        }
    } else {
        None
    }
}

fn union<'a>(
    mut value_map: &mut HashMap<&'a str, (&'a str, f64)>,
    dividend: &'a str,
    divisor: &'a str,
    quotient: f64,
) -> () {
    let group_id_dividend = find(&mut value_map, dividend);
    let group_id_divisor = find(&mut value_map, divisor);

    value_map.insert(
        group_id_dividend.unwrap(),
        (group_id_divisor.unwrap(), quotient),
    );
}

fn connected<'a>(value_map: &mut HashMap<&'a str, (&'a str, f64)>, x: &'a str, y: &'a str) -> bool {
    println!("CALLING CONNECTED");
    println!("CALLING FIND: x = {}", x);
    println!("CALLING FIND: y = {}", y);
    match (find(value_map, x), find(value_map, y)) {
        (Some(root_x), Some(root_y)) => root_x.eq(root_y),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let queries = Vec::from([["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let values = vec![2.0, 3.0];
        let equations = Vec::from([["a", "b"], ["b", "c"]])
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

    #[test]
    fn example_4() -> () {
        let equations = Vec::from([["x1", "x2"], ["x2", "x3"], ["x3", "x4"], ["x4", "x5"]])
            .iter()
            .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
            .collect();
        let values = vec![3.0, 4.0, 5.0, 6.0];
        let queries = Vec::from([
            ["x1", "x5"],
            ["x5", "x2"],
            ["x2", "x4"],
            ["x2", "x2"],
            ["x2", "x9"],
            ["x9", "x9"],
        ])
        .iter()
        .map(|edge| edge.iter().map(|str| String::from(*str)).collect())
        .collect();
        let result = Solution::calc_equation(equations, values, queries);

        assert_eq!(
            result,
            [360.00000, 0.00833, 20.00000, 1.00000, -1.00000, -1.00000]
        );
    }
}
