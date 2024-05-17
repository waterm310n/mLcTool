package ds

type pair struct {
	fa   int //父亲
	size int // size
}

type UnionFind struct {
	Fa     []pair
	Groups int // 连通分量个数
}

// 创建并查集
//
// n: 图中节点的数量
func NewUnionFind(n int) UnionFind {
	fa := make([]pair, n) // n+1
	for i := range fa {
		fa[i] = pair{i, 1}
	}
	return UnionFind{fa, n}
}

// 递归版本,根据x找到其连通分量的根
func (u UnionFind) FindR(x int) pair {
	if u.Fa[x].fa != x {
		u.Fa[x] = u.FindR(u.Fa[x].fa) //路径压缩
	}
	return u.Fa[x]
}

// 合并
func (u *UnionFind) Merge(from, to int) {
	x, y := u.FindR(from), u.FindR(to)
	if x.fa == y.fa {
		return
	}
	u.Fa[x.fa].fa = y.fa
	u.Fa[y.fa].size += u.Fa[x.fa].size
	u.Groups--
}

// 检查点x与点y是否连通
func (u UnionFind) Same(x, y int) bool {
	return u.FindR(x).fa == u.FindR(y).fa
}