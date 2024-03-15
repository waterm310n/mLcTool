package lctoolgo

import (
	"strconv"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 读取树序列，生成二叉树
func GenerateBinaryTree(sequence string) *TreeNode {
	if sequence == "[]" {
		return nil
	}
	vals := strings.Split(sequence[1:len(sequence)-1], ",")
	val, _ := strconv.Atoi(vals[0])
	root := &TreeNode{Val: val}
	queue := []*TreeNode{root}
	for i := 1; i < len(vals); i += 2 {
		cur := queue[0]
		queue = queue[1:]
		val, err := strconv.Atoi(vals[i])
		if err != nil {
			cur.Left = nil
		} else {
			cur.Left = &TreeNode{Val: val}
			queue = append(queue, cur.Left)
		}
		if i+1 < len(vals) {
			val, err = strconv.Atoi(vals[i+1])
			if err != nil {
				cur.Right = nil
			} else {
				cur.Right = &TreeNode{Val: val}
				queue = append(queue, cur.Right)
			}
		}
	}
	return root
}

type ListNode struct {
	Val  int
	Next *ListNode
}

// 读取链表序列，生成链表
func GenerateList(sequence string) *ListNode {
	if sequence == "[]" {
		return nil
	}
	vals := strings.Split(sequence[1:len(sequence)-1], ",")
	dummyHead := &ListNode{-1, nil}
	var prev, cur *ListNode = dummyHead, nil
	for _, val := range vals {
		val, _ := strconv.Atoi(val)
		cur = &ListNode{Val: val}
		prev.Next = cur
		prev = cur
	}
	return dummyHead.Next
}
