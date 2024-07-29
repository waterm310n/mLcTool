// 全排列算法

// LCR 083. 全排列
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::permute_helper(& mut nums, 0, &mut res);
        res
    }

    fn permute_helper(from:&mut Vec<i32>,start:usize,result:&mut Vec<Vec<i32>>) {
        if start == from.len() {
            result.push(from.clone());
        }else {
            for i in start..from.len() {
                from.swap(start,i);
                Self::permute_helper(from,start+1,result);
                from.swap(start,i);
            }
        }
    }
}