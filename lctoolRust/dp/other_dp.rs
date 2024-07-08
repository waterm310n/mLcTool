// ======================== 其它dp ========================

// 629. K 个逆序对数组
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        // f(i,j) 表示以i结尾,存在j个逆序对的排列数量,此题必须要写前缀和优化的版本，所以不太好用记忆化写
        // 直接写递推式
        const MOD:i32 = 1e9 as i32 + 7;
        let mut dp:Vec<Vec<i32>> = vec![vec![0;1+k as usize];n as usize];
        dp[0][0] = 1;
        for i in 1..n as usize{
            let mut prefix = vec![0;k as usize+2];
            for j in 0..1 + k as usize {
                prefix[j+1] = (prefix[j] + dp[i-1][j]) % MOD;
            }
            for j in 0..1 + k as usize {
                dp[i][j] = ((prefix[j+1] - prefix[j-i.min(j)])%MOD + MOD) %MOD;
            }
        }
        return dp[n as usize-1][k as usize];
    }
}

// 100333. 统计逆序对的数目 
impl Solution {
    // 记忆化搜索版本
    pub fn number_of_permutations_memo_search(n:i32,requirements: Vec<Vec<i32>>) -> i32{
        // f(i,j) 表示以下标i结尾的排列中有j个逆序对的排列数量
        // req[i] 为 -1 表示以下标i结尾，可以有任意个逆序对，否则表示以下标i结尾，必须要有req[i]个逆序对
        // 当前时间复杂度为O(mn min(m,n))
        let mut req = vec![-1;n as usize];
        for requierment in requirements {
            req[requierment[0] as usize] = requierment[1];
        }
        if req[0] >= 1{ return 0; }
        let m = req[n as usize - 1] as usize;
        let mut memo:Vec<Vec<Option<i32>>> = vec![vec![None;m+1];n as usize ];
        fn dfs(i:usize,j:usize,memo:&mut Vec<Vec<Option<i32>>>,req: & Vec<i32>) -> i32 {
            const MOD:i32 = 1e9 as i32 + 7;
            if i == 0 { return if j == 0 { 1 } else { 0 }; }
            if let Some(res) = memo[i][j] { return res; }
            let mut res = 0;
            if req[i - 1] != -1 { //如果前一个下标有逆序对要求有req[i-1]个逆序对
                let r = req[i-1] as usize;
                // 如果r > j 显然不合法,如果r+j < j，则说明前面的逆序对太少了，也不合法
                return if r <= j && r + i >= j {dfs(i-1, r , memo, req)} else { 0 };
            }
            // 对于当前下标i,最多可以产生i个逆序对,同时要求j-i >= 0 => up = i.min(j)
            // 对于当前下标i,最少可以产生0个逆序对
            let (down,up) = (0,i.min(j));
            for k in down..=up {
                res = (res + dfs(i-1, j-k, memo, req))%MOD;
            }
            memo[i][j] = Some(res);
            return res;
        }
        dfs(n as usize - 1, m, &mut memo, &req)
    }
    

    pub fn number_of_permutations_recurrence(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        // 记忆化搜索转递推
        // f(i,j) = f(i-1,j-k) k in [0,i.min(j)]
        const MOD:i32 = 1e9 as i32 + 7;
        let mut req = vec![-1;n as usize];
        for requierment in requirements {
            req[requierment[0] as usize] = requierment[1];
        }
        if req[0] >= 1{ return 0; }
        let m = req[n as usize - 1] as usize;
        let mut dp = vec![vec![0;m+1];n as usize];
        dp[0][0] = 1;
        for i in 1..n as usize {
            let mut mx = m ;
            if req[i] != -1 {
                mx = req[i] as usize;
            }
            if req[i-1] != -1 {
                let r = req[i-1] as usize;
                for j in r..=mx.min(i+r) {
                    dp[i][j] = dp[i-1][r];
                }
            }else{
                if req[i] != -1 {
                    let (down,up) = (0,i.min(mx));
                    for k in down..=up {
                        dp[i][mx] = (dp[i-1][mx-k] + dp[i][mx])%MOD;
                    }
                }else{
                    for j in 0..=mx {
                        let (down,up) = (0,i.min(j));
                        // dp[i-1][j-up] + ... + dp[i-1][j] => 要计算到dp[i-1][mx]
                        for k in down..=up {
                            dp[i][j] = (dp[i-1][j-k] + dp[i][j])%MOD;
                        }
                    }
                }
            }
        }
        return dp[n as usize -1][m];
    }

    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        // 前缀和优化递推
        // f(i,j) = f(i-1,j-k) k in [0,i.min(j)]
        const MOD:i64 = 1e9 as i64 + 7;
        let mut req = vec![-1;n as usize];
        for requierment in requirements {
            req[requierment[0] as usize] = requierment[1];
        }
        if req[0] >= 1{ return 0; }
        let m = req[n as usize - 1] as usize;
        let mut dp = vec![vec![0;m+1];n as usize];
        dp[0][0] = 1;
        for i in 1..n as usize {
            let mut mx = m ;
            if req[i] != -1 {
                mx = req[i] as usize;
            }
            // 计算前缀和
            let mut prefix = vec![0;mx+2];
            for j in 0..=mx{
                prefix[j+1] = (prefix[j] as i64 + dp[i-1][j] as i64) % MOD;
            }
            if req[i-1] != -1 {
                let r = req[i-1] as usize;
                for j in r..=mx.min(i+r) {
                    dp[i][j] = dp[i-1][r];
                }
            }else{
                if req[i] != -1 {
                    let up = i.min(mx);
                    dp[i][mx] = ((prefix[mx+1] - prefix[mx-up]) % MOD + MOD)%MOD; 
                }else{
                    for j in 0..=mx {
                        let up= i.min(j);
                        dp[i][j] =((prefix[j+1] - prefix[j-up])% MOD + MOD)%MOD; 
                    }
                }
            }
        }
        return dp[n as usize -1][m] as i32;
    }
}