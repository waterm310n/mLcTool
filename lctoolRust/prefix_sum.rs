
// ========================前缀和+哈希 计数========================

// 930. 和相同的二元子数组 1592
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        // prefix[i]-prefix[j]==goal
        let mut mp = HashMap::new();
        let mut cur_sum = 0;
        let mut ans = 0;
        mp.insert(goal, 1);
        for num in nums {
            cur_sum += num;
            if let Some(&cnt) = mp.get(&cur_sum) {
                ans += cnt;
            }
            mp.entry(goal+cur_sum).and_modify(|x| *x +=1 ).or_insert(1);
        }
        return ans
    }
}

// 1524. 和为奇数的子数组数目 1611
impl Solution {
    pub fn num_of_subarrays(nums: Vec<i32>) -> i32 {
        // (prefix[i]-prefix[j])%2 == 1%2
        const MOD:i32 = 1_000_000_000+7;
        let mut mp = HashMap::new();
        let mut cur_sum = 0;
        let mut ans = 0;
        mp.insert(1, 1);
        for num in nums {
            cur_sum += num;
            if let Some(&cnt) = mp.get(&(cur_sum%2)) {
                ans = (ans+cnt)%MOD;
            }
            mp.entry((1+cur_sum)%2).and_modify(|x| *x += 1 ).or_insert(1);
        }
        return ans
    }
}

// 974. 和可被 K 整除的子数组 1676
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        // (prefix[i]-prefix[j])%k == 0
        let mut mp = HashMap::new();
        let mut cur_sum = 0;
        let mut ans = 0;
        mp.insert(0, 1);
        for num in nums {
            cur_sum += num;
            if let Some(&cnt) = mp.get(&((cur_sum%k+k)%k)) {
                ans += cnt;
            }
            mp.entry((cur_sum%k+k)%k).and_modify(|x| *x +=1 ).or_insert(1);
        }
        return ans
    }
}

// 437. 路径总和 III
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        // cur_sum = target_sum + pre_sum
        let mut mp = HashMap::new();
        mp.insert(target_sum as i64,1);
        let mut ans = 0;
        fn dfs(cur_node:&Option<Rc<RefCell<TreeNode>>>,
            mp:& mut HashMap<i64,i32>,
            cur_sum: i64,
            ans:&mut i32,
            target_sum: i64
            ) {
                if let Some(cur) = cur_node {
                    let node = cur.borrow();
                    let cur_sum = cur_sum + node.val as i64;
                    if let Some(cnt) = mp.get(&cur_sum) {
                        *ans += cnt;
                    }
                    mp.entry(cur_sum+target_sum).and_modify(|x| *x+=1).or_insert(1);
                    dfs(&node.left, mp, cur_sum, ans, target_sum);
                    dfs(&node.right, mp, cur_sum, ans, target_sum);
                    mp.entry(cur_sum+target_sum).and_modify(|x| *x-=1);
                }
        }
        dfs(&root, &mut mp,0i64, &mut ans, target_sum as i64);
        return ans
    }
}

// 2588. 统计美丽子数组数目 1697 前缀异或和与哈希
impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        // prefix[i]^prefix[j] == 0
        let mut mp = HashMap::new();
        let mut cur_xor_sum = 0;
        mp.insert(cur_xor_sum, 1);
        let mut ans = 0;
        for num in nums {
            cur_xor_sum ^= num;
            if let Some(&cnt) = mp.get(&cur_xor_sum) {
                ans += cnt;
            }
            mp.entry(cur_xor_sum).and_modify(|x| *x+=1).or_insert(1);
        }
        return ans
    }
}

// 525. 连续数组 前缀和+哈希 求最大长度
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        // 1)prefix[i]-prefix[j] = 0 2)nums[i] = 1 if nums[i] == 1 3)nums[i] = -1 if nums[i] == 0
        let mut mp = HashMap::new();
        let mut cur_sum = 0;
        let mut ans = 0;
        mp.insert(0, -1);
        for (index,num) in nums.iter().enumerate() {
            if *num == 0 {
                cur_sum += -1;
            }else{
                cur_sum += num;
            }
            if let Some(&prev_index) = mp.get(&cur_sum) {
                ans = ans.max(index as i32 -prev_index) ;
            }
            mp.entry(cur_sum).or_insert(index as i32);
        }
        return ans
    }
}

// 3026. 最大好子数组和 1817
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        // |nums[j]-nums[i]| == k 
        // nums[j] = nums[i]+k
        // nums[j] = nums[i]-k
        let mut mp:HashMap<i32, i64> = HashMap::new();
        let mut cur_sum :i64= 0;
        let mut ans = i64::MIN;
        for num in nums{
            mp.entry(num).and_modify(|x| *x = (*x).min(cur_sum)).or_insert(cur_sum as i64);
            cur_sum += num as i64;
            if let Some(x) = mp.get(&(num+k)) {
                ans = ans.max(cur_sum-x);
            }
            if let Some(x) = mp.get(&(num-k)) {
                ans = ans.max(cur_sum-x);
            }
        }
        if ans == i64::MIN{
            return 0
        }
        return  ans;
    }
}

// 1546. 和为目标值且不重叠的非空子数组的最大数目 1855
impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut st = HashSet::new();
        let mut cur_sum = 0;
        st.insert(cur_sum+target);
        let mut ans = 0;
        for num in nums {
            cur_sum += num;
            if st.contains(&cur_sum){
                ans += 1;
                st.clear();
            }
            st.insert(cur_sum+target);
        }
        ans
    }
}

// 面试题 17.05. 字母与数字
impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        // prefix_alpha_num[j]-prefix_alpha_num[i] = prefix_digit_num[j]-prefix_digit_num[i]
        // prefix_alpha_num[j]-prefix_digit_num[j] = prefix_alpha_num[i]-prefix_digit_num[i]
        // 故字母则加一，数字则减一
        let mut mp = HashMap::new();
        let mut cur_sum = 0;
        let (mut left,mut length) = (0,0);
        mp.insert(cur_sum, -1);
        for (index,elem) in array.iter().enumerate() {
            if let Ok(_) = elem.parse::<i32>() {
                cur_sum -= 1;
            }else{
                cur_sum += 1;
            }
            if let Some(&prev_index) = mp.get(&cur_sum) {
                if index as i64 - prev_index as i64 > length{
                    length = index as i64 - prev_index as i64;
                    left = prev_index+1;
                }else if index as i64 - prev_index as i64 == length{
                    left = left.min(prev_index+1);
                }
            }
            mp.entry(cur_sum).or_insert(index as i64);
        }
        array.into_iter().skip(left as usize).take(length as usize).collect()
    }
}

// 1124. 表现良好的最长时间段
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        // prefix[i]-prefix[j]>0 => prefix[i] = prefix[j] + 1 
        // 因为p[j]如果想要变得更小，那么p[j]的下标必然更大，因为前缀和是从0开始计算
        // 需要注意的是这个前缀和的利用情况仅仅发生在当前前缀和小于等于0时使用。
        let mut mp = HashMap::new();
        let mut cur_sum = 0;
        mp.insert(1, -1);
        let mut ans = 0;
        for (index,hour) in hours.iter().enumerate() {
            if *hour > 8 {
                cur_sum += 1;
            }else {
                cur_sum -= 1;
            }
            if cur_sum >= 1 {
                ans = ans.max(index as i32 +1);
            }
            if let Some(prev_index) = mp.get(&cur_sum) {
                ans = ans.max(index as i32 - prev_index);
            }
            mp.entry(cur_sum+1).or_insert(index as i32);
        }
        return ans
    }
}