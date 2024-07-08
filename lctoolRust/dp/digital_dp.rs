// ======================== 数位dp ========================

/*
用记忆化写好写，
记录is_limit 为true表示前面数字都顶满了，为false则表示可以随便填写
与is_num 为true表示前面有填过数字，为false表示前面没有填过数字(用于解决前导0)
*/

// 233. 数字 1 的个数
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let n : Vec<u8> = n.to_string().bytes().collect();
        let length = n.len();
        // memo[i][1_num][is_limit]
        let mut memo = vec![vec![vec![None;2];length];length];
        fn dfs(index:usize,one_nums:i32,is_limit:bool,is_num:bool,memo: &mut Vec<Vec<Vec<Option<i32>>>>,n:&Vec<u8>) -> i32{
            if index == n.len() {
                return if is_num { one_nums } else { 0 };
            }
            if let Some(res) = memo[index][one_nums as usize][if is_limit {1} else {0}] {
                return res
            }
            let mut res = 0;
            if !is_num {
                res = dfs(index+1, one_nums, false, is_num, memo, n);
            }
            let up = if is_limit { (n[index]-'0' as u8) as usize } else{ 9 };
            let down = if is_num { 0 }else{ 1 };
            for digit in down..=up {
                res += dfs(index+1, if digit == 1 {one_nums+1} else {one_nums} 
                    , is_limit && digit == up, true, memo, n)
            }
            memo[index][one_nums as usize][if is_limit {1} else {0}] = Some(res);
            res
        }
        return dfs(0,0,true,false,&mut memo,&n);
    }
}

// 2376. 统计特殊整数 2120
impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let n : Vec<u8> = n.to_string().bytes().collect();
        let length = n.len();
        // memo[length][is_limit][mask]
        let mut memo:Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None;1 << 10];2];length];
        // is_limit为true表示前面填写了与n相同的数字
        // is_num 为true表示前面有填过数字，为false表示前面没有填过数字
        fn dfs(i:usize,mask:usize,is_limit:bool,is_num:bool,memo: &mut Vec<Vec<Vec<Option<i32>>>>,length: usize,n:&Vec<u8>) -> i32 {
            if i == length {
                return if is_num { 1 }else { 0 }
            }
            if let Some(x) = memo[i][if is_limit {1} else {0}][mask] { 
                return x;
            }
            let mut res = 0;
            if !is_num {
                res = dfs(i+1, mask, false, is_num, memo, length,n);
            }
            let up = if is_limit { (n[i]-'0' as u8) as usize } else{ 9 };
            let down = if is_num { 0 }else{ 1 };
            for digit in down..=up {
                if (mask >> digit) & 1 == 0 {
                    res += dfs(i+1, mask | (1 << digit), is_limit && digit == up, true, memo, length, n);
                }
            }
            memo[i][if is_limit {1} else {0}][mask] = Some(res);
            return res
        }
        return dfs(0, 0, true, false, &mut memo, length, &n)
    }
}

