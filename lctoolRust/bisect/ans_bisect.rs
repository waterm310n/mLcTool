// -------------------- 二分猜答案 ------------------------------

// 778. Swim in Rising Water 2097
impl Solution {
    fn dfs(grid:&Vec<Vec<i32>>,is_visited:&mut Vec<Vec<bool>>,i:usize,j:usize,height:i32) -> bool {
        is_visited[i][j] = true;
        if i == grid.len()-1 && j == grid[0].len()-1 {
            return true;
        }
        for (next_i,next_j) in [(i.saturating_sub(1),j),(i,j.saturating_sub(1)),(i+1,j),(i,j+1)] {
            if next_i < grid.len() && next_j < grid[0].len() && !is_visited[next_i][next_j] 
                && (grid[next_i][next_j] == grid[i][j] || grid[next_i][next_j] != grid[i][j]  && grid[next_i][next_j] <= height && grid[i][j] <= height) {
                if Solution::dfs(grid,is_visited,next_i,next_j,height) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let (mut left, mut right) = (0, grid.iter().flatten().max().unwrap().to_owned());
        while left <= right {
            let mid = (left + right) >> 1;
            let mut is_visited = vec![vec![false; grid[0].len()]; grid.len()];
            if Solution::dfs(&grid, & mut is_visited, 0, 0, mid) {
                right = mid - 1;
            }else{
                left = mid + 1;
            }
        }
        left
    }
}

// 1552. Magnetic Force Between Two Balls 与2517题是一样的 1920
impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let (mut left,mut right) = (1,position.iter().max().unwrap().to_owned());
        
        let check = |limit| -> bool {
            // 两个球放的距离必须要大于等于limit，计算总共可以放多少个球
            // 如果可以放的球的数量最终大于等于m，则说明limit可以更大
            let mut acc = 0;
            let mut prev_pos = None;
            for pos in position.iter(){
                if let Some(pp) = prev_pos {
                    if pos-pp >= limit {
                        acc += 1;
                        prev_pos = Some(pos);
                    }
                }else{
                    acc += 1;
                    prev_pos = Some(pos);
                }
            }
            acc >= m
        };

        while left <= right {
            let mid = (left + right) >> 1;
            if check(mid) {
                left = mid + 1;
            }else{
                right = mid - 1;
            }
        }
        right
    }
}
