// -----------最短路--------------------

/*
迪杰斯特拉算法

略
*/

/*
多源BFS
input: grid: Vec<Vec<i32>> 其中值为1表示病毒起点，0表示没感染病毒
let mut distance_grid:Vec<Vec<Option<i32>>> = vec![vec![None;grid.len()];grid.len()]; //每个点到病毒起点的最短距离
let mut queue = vec![];
for (row_index,row) in grid.iter().enumerate(){
    for (col_index,val) in row.iter().enumerate() {
        if *val == 1 {
            distance_grid[row_index][col_index] = Some(0);
            queue.push((row_index,col_index));
        }
    }
}
let mut distance = 1;
while queue.len() != 0 {
    let temp = queue;
    queue = vec![];
    for (i,j) in temp {
        for (next_i,next_j) in [(i.saturating_sub(1),j),(i,j.saturating_sub(1)),(i+1,j),(i,j+1)] {
            if next_i < grid.len() && next_j < grid.len() && distance_grid[next_i][next_j].is_none() {
                distance_grid[next_i][next_j] = Some(distance);
                queue.push((next_i,next_j)); 
            }
        }
    }
    distance += 1;
}

*/

// 2812. Find the Safest Path in a Grid 2154 
// 多远BFS，二分
impl Solution {
    fn dfs(
        i: usize,
        j: usize,
        is_visited: &mut Vec<Vec<bool>>,
        distance_grid: &Vec<Vec<i32>>,
        limit: usize,
    ) -> bool {
        is_visited[i][j] = true;
        if distance_grid[i][j] < limit as i32{ //为了防止起点不符合条件，因此在函数的开头做合法性判断
            return false
        }
        if i == distance_grid.len()-1 && j == distance_grid[0].len() -1 {
            return true;
        }
        for (next_i, next_j) in [
            (i.saturating_sub(1), j),
            (i, j.saturating_sub(1)),
            (i + 1, j),
            (i, j + 1),
        ] {
            if next_i < distance_grid.len()
                && next_j < distance_grid[0].len()
                && !is_visited[next_i][next_j]
            {
                if Solution::dfs(next_i, next_j, is_visited, distance_grid, limit) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {

        // 预处理计算每个格子到1格子的距离
        let mut distance_grid: Vec<Vec<Option<i32>>> = vec![vec![None; grid.len()]; grid.len()];
        let mut queue = vec![];
        for (row_index, row) in grid.iter().enumerate() {
            for (col_index, val) in row.iter().enumerate() {
                if *val == 1 {
                    distance_grid[row_index][col_index] = Some(0);
                    queue.push((row_index, col_index));
                }
            }
        }
        let mut distance = 1;
        while queue.len() != 0 {
            let temp = queue;
            queue = vec![];
            for (i, j) in temp {
                for (next_i, next_j) in [
                    (i.saturating_sub(1), j),
                    (i, j.saturating_sub(1)),
                    (i + 1, j),
                    (i, j + 1),
                ] {
                    if next_i < grid.len()
                        && next_j < grid.len()
                        && distance_grid[next_i][next_j].is_none()
                    {
                        distance_grid[next_i][next_j] = Some(distance);
                        queue.push((next_i, next_j));
                    }
                }
            }
            distance += 1;
        }

        let distance_grid: Vec<Vec<i32>> = distance_grid
            .into_iter()
            .map(|row| row.into_iter().map(|val| val.unwrap()).collect())
            .collect();

        let (mut left, mut right) = (0, grid.len());
        while left <= right {
            let mid = (left + right) >> 1;
            let mut is_visited = vec![vec![false; grid.len()]; grid.len()];
            if Solution::dfs(0, 0, &mut is_visited, &distance_grid, mid) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right as i32
    }
}