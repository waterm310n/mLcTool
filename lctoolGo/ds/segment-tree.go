/*
摘抄自灵茶山艾府的线段树
*/

type seg []struct{
	l,r int //当前区间的左边界与右边界
	val int 
}

func (seg) mergeInfo(a,b int) int {
	return max(a,b)
}

func (t seg) set(o,val int){
	t[o].val = t.mergeInfo(t[o].val,val)
}