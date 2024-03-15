package lctoolgo

import (
	"testing"
)

// Morris前序遍历，摘自Leetcode官方题解
func preorderTraversal(root *TreeNode) (res []int) {
	var p1, p2 *TreeNode = root, nil
	for p1 != nil {
		p2 = p1.Left
		if p2 != nil {
			for p2.Right != nil && p2.Right != p1 {
				p2 = p2.Right
			}
			if p2.Right == nil {
				res = append(res, p1.Val)
				p2.Right = p1
				p1 = p1.Left
				continue
			}
			p2.Right = nil
		} else {
			res = append(res, p1.Val)
		}
		p1 = p1.Right
	}
	return
}

// Morris中序遍历，摘自Leetcode官方题解
func inorderTraversal(root *TreeNode) (res []int) {
	for root != nil {
		if root.Left != nil {
			// predecessor 节点表示当前 root 节点向左走一步，然后一直向右走至无法走为止的节点
			predecessor := root.Left
			for predecessor.Right != nil && predecessor.Right != root {
				// 有右子树且没有设置过指向 root，则继续向右走
				predecessor = predecessor.Right
			}
			if predecessor.Right == nil {
				// 将 predecessor 的右指针指向 root，这样后面遍历完左子树 root.Left 后，就能通过这个指向回到 root
				predecessor.Right = root
				// 遍历左子树
				root = root.Left
			} else { // predecessor 的右指针已经指向了 root，则表示左子树 root.Left 已经访问完了
				res = append(res, root.Val)
				// 恢复原样
				predecessor.Right = nil
				// 遍历右子树
				root = root.Right
			}
		} else { // 没有左子树
			res = append(res, root.Val)
			// 若有右子树，则遍历右子树
			// 若没有右子树，则整颗左子树已遍历完，root 会通过之前设置的指向回到这颗子树的父节点
			root = root.Right
		}
	}
	return
}

// 判断两个[]int的切片的值是否一一对应
func isValSame(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	if (a == nil) != (b == nil) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}

// 测试二叉树生成
func TestGenerateBinaryTree(t *testing.T) {
	testcases := []struct {
		in           string // 输入的二叉树序列
		wantPreorder []int  // 前序遍历结果
		wantInorder  []int  // 中序遍历结果
	}{
		{"[]", nil, nil},
		{"[1]", []int{1}, []int{1}},
		{"[1,null,2,3]", []int{1, 2, 3}, []int{1, 3, 2}},
		{"[1,null,0,0,1]", []int{1, 0, 0, 1}, []int{1, 0, 0, 1}},
		{"[1,1,0,1,1,0,1,0]", []int{1, 1, 1, 0, 1, 0, 0, 1}, []int{0, 1, 1, 1, 1, 0, 0, 1}},
	}
	for _, testcase := range testcases {
		root := GenerateBinaryTree(testcase.in)
		preorder := preorderTraversal(root)
		if !isValSame(preorder, testcase.wantPreorder) {
			t.Fatalf("in: %s,wantPreorder:%v ,get:%v", testcase.in, testcase.wantPreorder, preorder)
		}
		inorder := inorderTraversal(root)
		if !isValSame(inorder, testcase.wantInorder) {
			t.Fatalf("in: %s,wantInorder:%v ,get:%v", testcase.in, testcase.wantInorder, inorder)
		}
	}
}

// 遍历单向链表
func listNodeTraversal(head *ListNode) (res []int) {
	for head != nil {
		res = append(res, head.Val)
		head = head.Next
	}
	return
}

// 测试单向链表生成
func TestGenerateList(t *testing.T) {
	testcases := []struct {
		in   string // 输入的二叉树序列
		want []int  // 前序遍历结果
	}{
		{"[]", nil},
		{"[4,1,8,4,5]", []int{4, 1, 8, 4, 5}},
		{"[5,6,1,8,4,5]", []int{5, 6, 1, 8, 4, 5}},
	}
	for _, testcase := range testcases {
		head := GenerateList(testcase.in)
		res := listNodeTraversal(head)
		if !isValSame(res, testcase.want) {
			t.Fatalf("in: %s,wantPreorder:%v ,get:%v", testcase.in, testcase.want, res)
		}

	}
}
