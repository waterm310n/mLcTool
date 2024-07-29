// ======================== 最长递增子序列dp ========================

/*
Lis的复杂度可以使用贪心优化，可以在O(nlogn)的时间求出
*/

// 1671. 得到山形数组的最少删除次数 1913
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // 最长递增子序列，dp[i]表示以nums[i]结尾的最长递增子序列
        // 使用贪心策略优化最长递增子序列，时间复杂度可以从O(n^2)降低到O(nlogn))
        let mut lis_dp = vec![1; n];
        let mut g = vec![];
        for i in 0..n {
            let index = g.partition_point(|x| *x < nums[i]);
            if index == g.len(){
                g.push(nums[i]);
            }else{
                g[index] = nums[i];
            }
            lis_dp[i] = index+1;
        }
        // 最长递减子序列，dp[i]表示以nums[i]开头的最长递减子序列
        let mut lds_dp = vec![1; n];
        g = vec![];
        for i in (0..n).rev() {
            let index = g.partition_point(|x| *x < nums[i]);
            if index == g.len(){
                g.push(nums[i]);
            }else{
                g[index] = nums[i];
            }
            lds_dp[i] = index+1;
        }
        let mut ans = n;
        for i in 1..n {
            if lis_dp[i] >= 2 && lds_dp[i] >= 2 {
                ans = ans.min(n + 1 - lis_dp[i] - lds_dp[i] );
            }
        }
        ans as i32
    }
}