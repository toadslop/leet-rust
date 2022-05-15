use std::collections::LinkedList;

#[allow(dead_code)]
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let edgelist = flights.iter().fold(vec![vec![]; n as usize], into_edgelist);
        let mut data_table: Vec<Row> = vec![Row::new(-1, i32::MAX); n as usize];
        let mut tracker = Tracker::new(n as usize);

        tracker.push_back(src as usize);
        data_table[src as usize] = Row::new(0, 0);

        while let Some(airport_id) = tracker.pop_front() {
            let prev_stop_count = data_table[airport_id].stop_count;
            let prev_cost = data_table[airport_id].cost;
            println!("AIRPORT ID {}", airport_id);
            println!("PREV COST {}", prev_cost);
            println!("PREV STOP COUNT {}", prev_stop_count);
            for flight in edgelist[airport_id].iter() {
                println!("FLIGHT: {:?}", flight);
                if flight.weight + prev_cost < data_table[flight.end].cost
                    && (prev_stop_count < k || flight.end == dst as usize)
                {
                    println!("UPDATING");
                    data_table[flight.end].update(prev_cost + flight.weight, prev_stop_count + 1);

                    if !tracker.is_added(flight.end) {
                        tracker.push_back(flight.end);
                    }
                }
            }
            println!("DATA {:?}", data_table);
            println!("QUEUE{:?}", tracker.queue);
            println!("");
        }

        println!("DATA {:?}", data_table);

        if data_table[dst as usize].cost == i32::MAX {
            -1
        } else {
            data_table[dst as usize].cost
        }
    }
}

fn into_edgelist(mut edge_list: Vec<Vec<Edge>>, edge: &Vec<i32>) -> Vec<Vec<Edge>> {
    edge_list[edge[0] as usize].push(Edge::new(edge[1] as usize, edge[2]));
    edge_list
}

struct Tracker {
    queue: LinkedList<usize>,
    added: Vec<bool>,
}

impl Tracker {
    pub fn new(n: usize) -> Self {
        Tracker {
            queue: LinkedList::new(),
            added: vec![false; n],
        }
    }

    pub fn push_back(&mut self, item: usize) {
        self.queue.push_back(item);
        self.added[item] = true;
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        if let Some(item) = self.queue.pop_front() {
            self.added[item] = false;
            Some(item)
        } else {
            None
        }
    }

    pub fn is_added(&self, item: usize) -> bool {
        self.added[item]
    }
}

#[derive(Clone, Debug)]
struct Edge {
    end: usize,
    weight: i32,
}

impl Edge {
    pub fn new(end: usize, weight: i32) -> Self {
        Edge { end, weight }
    }
}

#[derive(Clone, Debug)]
struct Row {
    stop_count: i32,
    cost: i32,
}

impl Row {
    pub fn new(stop_count: i32, cost: i32) -> Self {
        Row { stop_count, cost }
    }

    fn set_cost(&mut self, cost: i32) {
        self.cost = cost;
    }

    fn set_stop_count(&mut self, stop_count: i32) {
        self.stop_count = stop_count;
    }

    pub fn update(&mut self, cost: i32, stop_count: i32) {
        self.set_cost(cost);
        self.set_stop_count(stop_count);
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
}

struct Solution {}
