struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, mut pipes: Vec<Vec<i32>>) -> i32 {
        for (house_id, cost) in wells.iter().enumerate() {
            pipes.push(vec![0, (house_id + 1) as i32, *cost]);
        }

        pipes.sort_by(|a, b| a[2].cmp(&b[2]));

        let mut rank = vec![0; (n + 1) as usize];
        let mut root = (0..=n as usize).collect::<Vec<usize>>();
        let mut min_cost = 0;

        for pipe in pipes.iter() {
            min_cost += union(&mut root, &mut rank, pipe);
        }

        min_cost
    }
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<i32>, pipes: &Vec<i32>) -> i32 {
    let root_x = find(root, pipes[0] as usize);
    let root_y = find(root, pipes[1] as usize);

    if root_x != root_y {
        if rank[root_x] > rank[root_y] {
            root[root_y] = root_x;
        } else {
            root[root_x] = root_y;
            if rank[root_x] == rank[root_y] {
                rank[root_y] += 1;
            }
        }
        pipes[2]
    } else {
        0
    }
}

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if x != root[x] {
        root[x] = find(root, root[x]);
    }
    root[x]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 3;
        let wells = vec![1, 2, 2];
        let pipes = Vec::from([[1, 2, 1], [2, 3, 1]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let result = Solution::min_cost_to_supply_water(n, wells, pipes);

        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() -> () {
        let n = 2;
        let wells = vec![1, 1];
        let pipes = Vec::from([[1, 2, 1], [1, 2, 2]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let result = Solution::min_cost_to_supply_water(n, wells, pipes);

        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() -> () {
        let n = 5;
        let wells = vec![46012, 72474, 64965, 751, 33304];
        let pipes = Vec::from([[2, 1, 6719], [3, 2, 75312], [5, 3, 44918]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let result = Solution::min_cost_to_supply_water(n, wells, pipes);

        assert_eq!(result, 131704);
    }

    #[test]
    fn example_4() -> () {
        let n = 9;
        let wells = vec![58732, 77988, 55446, 79246, 8265, 30789, 39905, 79968, 61679];
        let pipes = Vec::from([
            [2, 1, 45475],
            [3, 2, 41579],
            [4, 1, 79418],
            [5, 2, 17589],
            [7, 5, 4371],
            [8, 5, 82103],
            [9, 7, 55500],
        ])
        .iter()
        .map(|arr| Vec::from(*arr))
        .collect();

        let result = Solution::min_cost_to_supply_water(n, wells, pipes);

        assert_eq!(result, 362782);
    }

    #[test]
    fn example_5() -> () {
        let n = 6;
        let wells = vec![4625, 65696, 86292, 68291, 37147, 7880];
        let pipes = Vec::from([
            [2, 1, 79394],
            [3, 1, 45649],
            [4, 1, 75810],
            [5, 3, 22340],
            [6, 1, 6222],
        ])
        .iter()
        .map(|arr| Vec::from(*arr))
        .collect();

        let result = Solution::min_cost_to_supply_water(n, wells, pipes);

        assert_eq!(result, 204321);
    }

    #[test]
    fn example_6() -> () {
        let n = 50;
        let wells = vec![
            62693, 87782, 78682, 81671, 24745, 65255, 78647, 44719, 8331, 90816, 72429, 27535,
            38548, 26329, 56884, 61337, 54924, 89648, 60045, 68882, 8146, 86370, 88355, 17526,
            26113, 49779, 43781, 88498, 92375, 4917, 14854, 54266, 55725, 75875, 40380, 56552,
            19520, 40417, 10838, 38815, 17422, 22943, 96316, 109, 92321, 12420, 21717, 4707, 68733,
            70199,
        ];
        let pipes = Vec::from([
            [2, 1, 11653],
            [5, 2, 13613],
            [6, 3, 63921],
            [7, 4, 20988],
            [8, 5, 34053],
            [9, 2, 57779],
            [11, 5, 39145],
            [12, 10, 96742],
            [13, 7, 2696],
            [15, 4, 93103],
            [16, 11, 76857],
            [17, 3, 38026],
            [18, 7, 50670],
            [19, 14, 51565],
            [20, 5, 21487],
            [22, 13, 79172],
            [23, 3, 58291],
            [24, 17, 23634],
            [25, 8, 66553],
            [26, 19, 84892],
            [27, 20, 17595],
            [28, 4, 88853],
            [29, 5, 78073],
            [31, 22, 59100],
            [32, 29, 32588],
            [33, 25, 43304],
            [34, 31, 16139],
            [35, 3, 71565],
            [36, 26, 6076],
            [37, 3, 70712],
            [38, 26, 73943],
            [39, 8, 551],
            [40, 30, 84528],
            [41, 34, 31055],
            [42, 18, 43503],
            [43, 19, 56259],
            [44, 8, 36845],
            [45, 44, 69872],
            [46, 18, 86220],
            [47, 35, 55960],
            [48, 37, 51081],
            [50, 28, 69716],
        ])
        .iter()
        .map(|arr| Vec::from(*arr))
        .collect();

        let result = Solution::min_cost_to_supply_water(n, wells, pipes);

        assert_eq!(result, 1638068);
    }
}
