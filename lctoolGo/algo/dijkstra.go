package algo

import (
	"container/heap"
	"math"
)

// 堆摘抄自https://pkg.go.dev/container/heap，没有保留下标
// An Item is something we manage in a priority queue.
type Item struct {
	value    int // The value of the item; arbitrary.
	priority int    // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	item := x.(*Item)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	*pq = old[0 : n-1]
	return item
}

// 稀疏图，无负数的最短路算法。O(mlogm) m是边的数量
func Dijkstra(n int, edges [][]int) []int {
	mp := make([][]Item,n)
	for i := range edges {
		u,v,weight:= edges[i][0],edges[i][1],edges[i][2]
		mp[u] = append(mp[u], Item{v,weight})
		mp[v] = append(mp[v], Item{u,weight})
	}
	minHeap := PriorityQueue{{0,0}}
	dist := make([]int,n)
	for i := range dist {
		dist[i] = math.MaxInt
	}
	dist[0] = 0
	for minHeap.Len() > 0 {
		item := heap.Pop(&minHeap).(*Item)
		if item.priority > dist[item.value] { //这一行不能缺少
			continue
		}	
		for _,edge:= range mp[item.value]{
			if edge.priority+item.priority < dist[edge.value]{
				dist[edge.value] = edge.priority+item.priority
				heap.Push(&minHeap,&Item{edge.value,dist[edge.value]})
			}
		}
	}
	return dist
}