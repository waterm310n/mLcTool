// ========================分组循环题单========================

/* 摘自灵神 O(n)
外层循环负责遍历组之前的准备工作（记录开始位置），和遍历组之后的统计工作（更新答案最大值）。
内层循环负责遍历组，找出这一组最远在哪结束。
n = len(nums)
i = 0
while i < n: // 外层循环
    start = i
    while i < n and ...: // 内层循环
        i += 1
    # 从 start 到 i-1 是一组
    # 下一组从 i 开始，无需 i += 1
*/

// 1869. 哪种连续子字符串更长 1205
pub fn check_zero_ones(s: String) -> bool {
    let s :Vec<char> = s.chars().collect();
    let (mut z_cnt,mut o_cnt) = (0,0);
    let mut i = 0;
    while i < s.len(){
        let start = i ;
        i += 1;
        while i < s.len(){
            if s[i] == s[i-1] {
                i +=1 ;
            }else{
                break;
            }
        }
        if s[start] == '1' {
            o_cnt = o_cnt.max(i-start);
        }else{
            z_cnt = z_cnt.max(i-start);
        }
    }
    return o_cnt > z_cnt
}

//1957. 删除字符使字符串变好 1358
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::new();
        let s : Vec<char>= s.chars().collect();
        let mut i = 0;
        while i < s.len(){
            let start = i;
            ans.push(s[i]);
            i += 1;
            while i < s.len() {
                if s[i] == s[i-1] {
                    if  i-start <= 1 {
                        ans.push(s[i])
                    }
                    i += 1;
                }else {
                    break
                }
            }
        }
        return ans;
    }
}

// 2110. 股票平滑下跌阶段的数目 1408
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut i = 0;
        while i < prices.len(){
            let start = i;
            i += 1;
            while i < prices.len() {
                if prices[i] + 1 == prices[i-1] {
                    i += 1;
                }else{
                    break;
                }
            }
            ans += (i-start+1)*(i-start)/2;
        }
        ans as i64
    }
}

// 2760. 最长奇偶子数组 1420
pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    while i < nums.len(){
        if nums[i] %2 != 0 || nums[i] > threshold{
            i += 1;
            continue;
        }
        let start = i;
        i += 1;
        while i < nums.len() {
            if nums[i] % 2 != nums[i-1]%2 && nums[i] <= threshold{
                i+=1
            }else{
                break;
            }
        }
        ans = ans.max(i as i32 - start as i32);
    }
    ans
}

// 2038. 如果相邻两个颜色均相同则删除当前颜色 1468
pub fn winner_of_game(colors: String) -> bool {
    let (mut a_cnt,mut b_cnt) = (0,0);
    let mut i = 0;
    let colors:Vec<u8> = colors.bytes().collect();
    while i < colors.len(){
        let start = i;
        i += 1;
        while i < colors.len(){
            if colors[i] == colors[i-1] {
                i += 1;
            }else{
                break;
            }
        }
        if colors[start] == 'A' as u8 {
            if i > start+2 {
                a_cnt += i-start-2;
            }
        }else{
            if i > start+2 {
                b_cnt += i-start-2;
            }
        }
    }
    return a_cnt > b_cnt
}

// 1839. 所有元音按顺序排布的最长子字符串 1580
pub fn longest_beautiful_substring(word: String) -> i32 {
    let s :Vec<u8> = word.bytes().collect();
    let mut ans = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] != b'a'{
            i += 1;
            continue;
        }
        let start = i;
        i += 1;
        let mut cnt = 1;
        while i < s.len() {
            if s[i] > s[i-1] {
                cnt += 1;
                i += 1;
            }else if s[i] == s[i-1]{
                i += 1;
            }else{
                break;
            }
        }
        if cnt == 5 {
            ans = ans.max(i-start)
        }
    }
    ans as i32
}
