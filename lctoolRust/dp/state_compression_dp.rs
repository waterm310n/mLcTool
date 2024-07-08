// ======================== 状压 dp ========================

// 526. 优美的排列
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let u = (1 << n) - 1;
        let mut memo:Vec<Option<i32>> = vec![None;u + 1];
        fn dfs(s:usize,memo:&mut Vec<Option<i32>>,u: usize,n: i32) -> i32 {
            if s == u {
                return 1
            }
            if let Some(res) = memo[s] {
                return res
            }
            let mut res = 0 ;
            let i = s.count_ones() + 1;
            for digit in 1..=n {
                if (s >> (digit-1) & 1 != 1 ) && (digit as u32 % i == 0 || i % digit as u32 == 0 ) {
                    res += dfs(s | ( 1 << (digit-1)), memo, u, n)
                }
            }
            memo[s] = Some(res);
            return res;
        }
        dfs(0, &mut memo,u,n)
    }
}

// 2741. 特别的排列 2012
impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        const MOD:i32 = 1e9 as i32+7;
        let u = (1 << nums.len()) - 1;
        let mut memo = vec![vec![None;nums.len()];u];
        fn dfs(s:usize,prev_index:usize,u:usize,nums:&Vec<i32>,memo:&mut Vec<Vec<Option<i32>>>) -> i32 {
            if s == u {
                return 1;
            }
            if let Some(res) = memo[s][prev_index] {
                return res
            }
            let mut res = 0;
            for index in 0..nums.len(){
                if (s >> index & 1 == 0) && (nums[index] % nums[prev_index] == 0 || nums[prev_index] % nums[index] == 0){
                    res = (res + dfs(s | 1 << index, index, u, nums, memo))%MOD;
                }
            }
            memo[s][prev_index] = Some(res);
            return res
        }
        let mut ans = 0;
        for index in 0..nums.len() {
            ans = (ans + dfs(1 << index, index, u, &nums, & mut memo))%MOD;
        }
        ans
    }
}

// 3149. 找出分数最低的排列 2642
impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let u = (1 << n) - 1;
        let mut memo = vec![vec![None;n];u];
        fn dfs(s:usize,prev_index:usize,u:usize,memo:&mut Vec<Vec<Option<i32>>>,nums: &Vec<i32>) -> i32 {
            if s == u { return nums[0].abs_diff(prev_index as i32) as i32}
            if let Some(res) = memo[s][prev_index] {
                return res
            }
            let mut res = i32::MAX;
            for index in 1..nums.len() {
                if (s >> index) & 1 == 0 {
                    res = res.min(dfs(s | (1 << index), index, u, memo, nums)+nums[index].abs_diff(prev_index as i32) as i32)
                }
            }
            memo[s][prev_index] = Some(res);
            return res;
        }
        let mut ans = vec![0];
        fn maks_ans(s:usize,prev_index:usize,u:usize,ans:&mut Vec<i32>,memo: &mut Vec<Vec<Option<i32>>>,nums: &Vec<i32>) {
            if s == u { return }
            let final_res = dfs(s, prev_index, u, memo, nums);
            for index in 1..nums.len() {
                if (s >> index) & 1 == 0 && dfs(s | ( 1 << index), index, u, memo, nums) + nums[index].abs_diff(prev_index as i32) as i32 == final_res {
                    ans.push(index as i32);
                    maks_ans(s | ( 1 << index), index, u, ans, memo, nums);
                    break
                }
            }
        }
        maks_ans(1, 0,u, &mut ans, &mut memo,&nums);
        return ans
    }
}