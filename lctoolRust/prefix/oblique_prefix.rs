// ========================斜向前缀和========================

/*
模板：

let (m,n) = (grid.len(),grid[0].len());
let mut p_diagonal = vec![vec![0;n+1];m+1]; // 主对角线
let mut s_diagonal = vec![vec![0;n+1];m+1]; // 次对角线
for (row_index,row) in grid.iter().enumerate() {
    for (col_index,val) in row.iter().enumerate() {
        p_diagonal[row_index+1][col_index+1] = p_diagonal[row_index][col_index]+val;
        s_diagonal[row_index+1][col_index] = s_diagonal[row_index][col_index+1]+val;
    } 
}

// 从 (x,y) 开始，向 ↘，连续的 k 个数的和
let query_diagonal = |x:usize, y:usize, k:usize| { return p_diagonal[x+k][y+k] - p_diagonal[x][y] };
// 从 (x,y) 开始，向 ↙，连续的 k 个数的和
let query_sub_diagonal = |x:usize, y:usize, k:usize| { return s_diagonal[x+k][y+1-k] - s_diagonal[x][y+1] };

*/


// 1878. 矩阵中最大的三个菱形和 1898
impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut x,mut y,mut z) = (0,0,0); //依次表示最大，第二大，第三大
        // 如果相等则不更新
        let mut update = | v | {
            if v > x {
                (x,y,z) = (v,x,y)
            }else if v < x && v > y {
                (y,z) = (v,y)
            }else if v < y && v > z{
                z = v
            }
        };
        let (m,n) = (grid.len(),grid[0].len());
        let mut p_diagonal = vec![vec![0;n+1];m+1]; // 主对角线
        let mut s_diagonal = vec![vec![0;n+1];m+1]; // 次对角线
        for (row_index,row) in grid.iter().enumerate() {
            for (col_index,val) in row.iter().enumerate() {
                p_diagonal[row_index+1][col_index+1] = p_diagonal[row_index][col_index]+val;
                s_diagonal[row_index+1][col_index] = s_diagonal[row_index][col_index+1]+val;
            } 
        }
	    // 从 (x,y) 开始，向 ↘，连续的 k 个数的和
	    let query_diagonal = |x:usize, y:usize, k:usize| { return p_diagonal[x+k][y+k] - p_diagonal[x][y] };
	    // 从 (x,y) 开始，向 ↙，连续的 k 个数的和
	    let query_sub_diagonal = |x:usize, y:usize, k:usize| { return s_diagonal[x+k][y+1-k] - s_diagonal[x][y+1] };

         
        // 暴力遍历坐标
        for row_index in 0..m {
            for col_index in 0..n {
                update(grid[row_index][col_index]);
                let mut length = 1;
                // 中心扩展遍历
                while length <= row_index && length+row_index < m && length <= col_index && length+col_index < n{
                    let a = query_diagonal(row_index-length,col_index,length); // 右上
                    let c = query_sub_diagonal(row_index-length+1,col_index-1,length);//左上
                    let b = query_diagonal(row_index+1,col_index-length+1,length); //左下
                    let d = query_sub_diagonal(row_index,col_index+length,length); //右下
                    update(a+b+c+d);
                    length += 1;
                }
            }
        }

        let mut ans = vec![x,y,z];
        while ans[ans.len()-1] == 0 {
            ans.pop();
        }
        return ans;
    }
}