use std::cmp::min;

#[allow(dead_code)]
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut previous = Vec::with_capacity(n as usize);
        let mut current = Vec::with_capacity(n as usize);

        for _ in (0..n as usize).into_iter() {
            previous.push(i32::MAX);
            current.push(i32::MAX);
        }

        previous[src as usize] = 0;
        current[src as usize] = 0;

        for _ in (0..k + 1).into_iter() {
            for flight in flights.iter() {
                let start = flight[0];
                let end = flight[1];
                let cost = flight[2];
                if previous[start as usize] < i32::MAX {
                    current[end as usize] =
                        min(previous[start as usize] + cost, current[end as usize]);
                }
            }
            previous = current.clone();
        }

        if current[dst as usize] == i32::MAX {
            -1
        } else {
            current[dst as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let flights: Vec<Vec<i32>> = [
            [0, 1, 100],
            [1, 2, 100],
            [2, 0, 100],
            [1, 3, 600],
            [2, 3, 200],
        ]
        .iter()
        .map(|array| Vec::from(*array))
        .collect();
        let src = 0;
        let dst = 3;
        let k = 1;

        let expected_result = 700;
        let actual_result = Solution::find_cheapest_price(n, flights, src, dst, k);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let flights: Vec<Vec<i32>> = [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
            .iter()
            .map(|array| Vec::from(*array))
            .collect();
        let src = 0;
        let dst = 2;
        let k = 1;

        let expected_result = 200;
        let actual_result = Solution::find_cheapest_price(n, flights, src, dst, k);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_3() {
        let n = 3;
        let flights: Vec<Vec<i32>> = [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
            .iter()
            .map(|array| Vec::from(*array))
            .collect();
        let src = 0;
        let dst = 2;
        let k = 0;

        let expected_result = 500;
        let actual_result = Solution::find_cheapest_price(n, flights, src, dst, k);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_4() {
        let n = 4;
        let flights: Vec<Vec<i32>> = [[0, 1, 1], [0, 2, 5], [1, 2, 1], [2, 3, 1]]
            .iter()
            .map(|array| Vec::from(*array))
            .collect();
        let src = 0;
        let dst = 3;
        let k = 1;

        let expected_result = 6;
        let actual_result = Solution::find_cheapest_price(n, flights, src, dst, k);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_5() {
        let n = 5;
        let flights: Vec<Vec<i32>> = [[0, 1, 1], [0, 2, 5], [1, 2, 1], [2, 3, 1], [3, 4, 1]]
            .iter()
            .map(|array| Vec::from(*array))
            .collect();
        let src = 0;
        let dst = 4;
        let k = 2;

        let expected_result = 7;
        let actual_result = Solution::find_cheapest_price(n, flights, src, dst, k);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_6() {
        let n = 7;
        let flights: Vec<Vec<i32>> = [
            [0, 3, 7],
            [4, 5, 3],
            [6, 4, 8],
            [2, 0, 10],
            [6, 5, 6],
            [1, 2, 2],
            [2, 5, 9],
            [2, 6, 8],
            [3, 6, 3],
            [4, 0, 10],
            [4, 6, 8],
            [5, 2, 6],
            [1, 4, 3],
            [4, 1, 6],
            [0, 5, 10],
            [3, 1, 5],
            [4, 3, 1],
            [5, 4, 10],
            [0, 1, 6],
        ]
        .iter()
        .map(|array| Vec::from(*array))
        .collect();
        let src = 2;
        let dst = 4;
        let k = 1;

        let expected_result = 16;
        let actual_result = Solution::find_cheapest_price(n, flights, src, dst, k);

        assert_eq!(expected_result, actual_result);
    }
}

struct Solution {}
