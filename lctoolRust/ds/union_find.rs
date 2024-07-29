/// ========================并查集========================

#[derive(Debug,Clone,Copy)]
struct Pair {
    /// 父亲
    pub fa : usize, 
    /// 集合大小
    size : usize, 
}

pub struct UnionFind {
    /// 连通分量个数
    pub groups : usize, 
    fa : Vec<Pair>,
}


impl UnionFind {

    /// 创建并查集
    /// n：图中节点的数量
    pub fn new(n : usize) -> Self {
        let mut fa = vec![Pair{fa: 0, size: 0}; n];
        for i in 0..n {
            fa[i].fa = i;
            fa[i].size = 1;
        }
        UnionFind {
            groups: n,
            fa,
        }
    }

    /// 递归版本,根据x找到其连通分量的根,同时进行路径压缩
    pub fn find_r(&mut self, x : usize) -> Pair {
        if self.fa[x].fa != x {
            self.fa[x].fa = self.find_r(self.fa[x].fa).fa;
        }
        return self.fa[x];
    }

    /// 合并连通分量
    pub fn merge(&mut self, from : usize, to : usize){
        let (x,y) = (self.find_r(from),self.find_r(to));
        if x.fa == y.fa { // 如果已经处在同一连通分量，直接返回
            return;
        }
        self.fa[y.fa].size += self.fa[x.fa].size;
        self.fa[x.fa].fa = y.fa;
        self.groups -= 1;
    }

    /// 判断两个节点是否在同一连通分量中
    pub fn same(&mut self, x : usize, y : usize) -> bool{
        return self.find_r(x).fa == self.find_r(y).fa;
    }
}

// 2709. 最大公约数遍历 质因数分解+并查集 2172
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true
        }
        let mut uf = UnionFind::new(nums.len());
        let mut mp = HashMap::new();
        for (index,mut num) in nums.into_iter().enumerate() {
            // 质因数分解
            if num == 1 {
                return false
            }
            let mut i = 2 ;
            while i * i <= num {
                if num % i == 0 {
                    if mp.contains_key(&i) { //不是首次出现，则将其与先前的节点进行合并
                        uf.merge(index, mp[&i]);
                    } else { //首次出现
                        mp.insert(i, index);
                    }
                    while num % i == 0 {
                        num /= i;
                    }
                }
                i += 1;
            }
            if num > 1 {
                if mp.contains_key(&num) {
                    uf.merge(index, mp[&num]);
                } else {
                    mp.insert(num, index);
                }
            }
        }
        uf.groups == 1
    }
}