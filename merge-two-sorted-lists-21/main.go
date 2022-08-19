package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	head := new(ListNode)
	temp := head

	for list1 != nil && list2 != nil {
		if list1.Val < list2.Val {
			temp.Next = list1
			list1 = list1.Next
		} else {
			temp.Next = list2
			list2 = list2.Next
		}
		temp = temp.Next
	}

	if list1 == nil {
		temp.Next = list2
	} else {
		temp.Next = list1
	}

	return head.Next
}

func printLinkedList(node *ListNode) {
	if node == nil {
		fmt.Println(nil)
		return
	}
	if node.Next != nil {
		printLinkedList(node.Next)
	}

	fmt.Println(node.Val)

}

func main() {
	// a := ListNode{4, nil}
	// b := ListNode{2, &a}
	// list1 := &ListNode{1, &b}

	// aa := ListNode{4, nil}
	// bb := ListNode{3, &aa}
	// list2 := &ListNode{1, &bb}

	var list1 *ListNode = nil
	var list2 *ListNode = &ListNode{0, nil}

	mergedList := mergeTwoLists(list1, list2)
	printLinkedList(mergedList)
}
