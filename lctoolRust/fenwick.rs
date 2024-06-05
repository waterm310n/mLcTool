/// ========================树状数组========================
/// 个人理解树状数组的优点在于动态计算前缀和
/// 即：能够对前缀和进行单点更新O(logn)，区间查询O(logn)


/* 从灵神的树状数组模板修改*/
struct Fenwick{
    inner: Vec<i32>
}

impl Fenwick{
    const FENWICK_INIT_VAL:i32 = 0; // -1e18
    pub fn new(n:usize) -> Self{
        Self{
            inner: vec![Self::FENWICK_INIT_VAL;n+1]
        }        
    }

    fn op(&self,a:i32, b:i32) -> i32 {
        return a + b // max(a, b)
    }

    // a[i] 增加 val
    // 1<=i<=n
    pub fn update(&mut self,mut i:i32, val:i32) {
        while i < self.inner.len() as i32{
            self.inner[i as usize] = self.op(self.inner[i as usize], val);
            i += i & -i; // lowbit
        }
    }

    // 求前缀和 a[1] + ... + a[i]
    // 1<=i<=n
    pub fn pre(&self,mut i:i32) -> i32 {
        let mut res = Self::FENWICK_INIT_VAL;
        while i > 0 {
            res = self.op(res, self.inner[i as usize]);
            i &= i - 1;
        }
        return res
    }

    // 求区间和 a[l] + ... + a[r]
    // 1<=l<=r<=n
    pub fn query(&self,l:i32, r:i32) ->i32 {
        return self.pre(r) - self.pre(l-1)
    }
}

// 3072. 将元素分配到两个数组中 II 2053
impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        // 做一个离散化的处理，目的是节约内存
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        sorted_nums.dedup(); // 注意这个方法去重只能用在有序数组中
        let (mut fenwick1,mut fenwick2) = (Fenwick::new(sorted_nums.len()),Fenwick::new(sorted_nums.len()));
        let (mut arr1,mut arr2) = (vec![nums[0]],vec![nums[1]]);
        fenwick1.update(sorted_nums.binary_search(&nums[0]).unwrap() as i32+1, 1);
        fenwick2.update(sorted_nums.binary_search(&nums[1]).unwrap() as i32+1, 1);
        for num in nums.iter().skip(2) {
            let i = sorted_nums.binary_search(num).unwrap() as i32;
            let l1 = arr1.len() - fenwick1.pre(i+1) as usize;
            let l2 = arr2.len() - fenwick2.pre(i+1) as usize;
            if l1 > l2 {
                arr1.push(*num);
                fenwick1.update(i+1, 1);
            }else if l1 < l2 {
                arr2.push(*num);
                fenwick2.update(i+1, 1);
            }else if arr1.len() > arr2.len(){
                arr2.push(*num);
                fenwick2.update(i+1, 1);
            }else{
                arr1.push(*num);
                fenwick1.update(i+1, 1);
            }
        }
        return arr1.into_iter().chain(arr2.into_iter()).collect();
    }
}