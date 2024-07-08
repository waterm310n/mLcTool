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

//2488. 统计中位数为 K 的子数组 1999
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        // 将nums进行处理，如果值大于k，则+1，小于k则-1
        // prefix[i]-prefix[j] == 0 或 prefix[i] - prefix[j] = 1 
        let mut mp:HashMap<_, _> = HashMap::new();
        let mut ans = 0;
        let mut cur_sum = 0;
        let mut k_pos = None;
        mp.insert(0, 1);
        for (index,num) in nums.iter().enumerate() {
            if *num > k {
                cur_sum += 1;
            }else if *num < k{
                cur_sum -= 1;
            }else{
                k_pos = Some(index);
            }
            if let Some(_) = k_pos { // 保证计算的区间和是包含k的
                if let Some(cnt) = mp.get(&cur_sum){
                    ans += cnt;
                }
                if let Some(cnt) = mp.get(&(cur_sum-1)){
                    ans += cnt;
                }
            }else{//只记录在遇到k之前的前缀和
                mp.entry(cur_sum).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        return ans;
    }
}

// 1590. 使数组和能被 P 整除 2039
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        // (total-(cur_sum-pre_sum))%p=0%p
        // total+pre_sum=cur_sum
        let p = p as i64;
        let total:i64 = nums.iter().map(|x| *x as i64).sum();
        let mut mp = HashMap::new();
        let mut ans = nums.len() as i32;
        let mut cur_sum = 0;
        mp.insert((total+cur_sum)%p, -1);
        for (index,&num) in nums.iter().enumerate() {
            cur_sum += num as i64;
            mp.insert((total+cur_sum)%p, index as i32);
            if let Some(prev_index) = mp.get(&(cur_sum%p)) {
                ans = ans.min(index as i32-prev_index);
            }
        }
        if ans == nums.len() as i32{
            return -1
        }
        return ans;
    }
}

// 2845. 统计趣味子数组的数目 2073
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        // nums[i] = 1 if nums[i]%modulo == k
        // (prefix[i] - prefix[j])%modulo == k
        // (prefix[i]-k)%modulo = prefix[j]%modulo 
        let mut mp:HashMap<_, _> = HashMap::new();
        let mut ans = 0;
        let mut cur_sum = 0;
        mp.insert(0, 1);
        for num in nums {
            if num%modulo == k {
                cur_sum += 1;
            }
            if let Some(cnt) = mp.get(&(((cur_sum-k)%modulo+modulo)%modulo)) {
                ans += cnt;
            }
            mp.entry(cur_sum).and_modify(|x| *x += 1).or_insert(1);
        }
        return ans
    }
}

// 1442. 形成两个异或相等数组的三元组数目 1528,O(n)做法应该有2100，比较综合
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        // 思路：a == b => a^b = 0 => prefix[i]^prefix[j] == 0 => prefix[i] == prefix[j] =>找出可行区间
        // 但是要求的是三元组，因此还需要知道下标才能计算出所有的组合。
        // 这一部分我看了题解，道理上是通过添加total来计算，它推了这个式子m*index-(i1+i2+i3+..im)
        // 因此我这里记录每一个下标,我的式子是sum0..m(index-i1-1)=m*index-(i1+i2+i3+..im)-1*m
        // 通过这个方式简化了计算，所以这道题还挺综合的
        let mut cnt_mp = HashMap::new();//统计有多少个m
        let mut total_mp = HashMap::new();
        let mut ans = 0;
        let mut cur_xor_sum = 0;
        cnt_mp.insert(0, 1);
        total_mp.insert(0, -1);
        for (index,num) in arr.iter().enumerate() {
            cur_xor_sum ^= num;
            // num不可能为0，因此prefix[i] = prefix[j]时 i >= j+2
            if let Some(cnt) = cnt_mp.get(&cur_xor_sum) { 
                let total = total_mp.get(&cur_xor_sum).unwrap();
                ans += cnt*(index as i32-1)-total;
            }
            cnt_mp.entry(cur_xor_sum).and_modify(|x| *x += 1).or_insert(1);
            total_mp.entry(cur_xor_sum).and_modify(|x| *x += index as i32).or_insert(index as i32);
        }
        return ans
    }
}