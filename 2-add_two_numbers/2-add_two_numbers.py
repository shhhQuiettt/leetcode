# Definition for singly-linked list.
from typing import Optional


class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def print_linked_list(l: Optional[ListNode]) -> None:
    while l is not None:
        print(l.val, end=" ")
        l = l.next
    print()


class Solution:
    def reverse_linked_list(self, l: Optional[ListNode]) -> Optional[ListNode]:
        prev = None
        while l is not None:
            next = l.next
            l.next = prev
            prev = l
            l = next

        return prev

    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:

        l1 = self.reverse_linked_list(l1)
        l2 = self.reverse_linked_list(l2)

        node = None
        carrier = 0
        while l1 is not None and l2 is not None:
            current_sum = l1.val + l2.val + carrier
            node = ListNode(val=current_sum % 10, next=node)
            carrier = current_sum // 10

            l1 = l1.next
            l2 = l2.next

        return node


def main():
    l3 = ListNode(3, None)
    l2 = ListNode(4, l3)
    l = ListNode(2, l2)

    k3 = ListNode(5, None)
    k2 = ListNode(6, k3)
    k = ListNode(4, k2)

    s = Solution()
    print_linked_list(s.addTwoNumbers(l, k))


if __name__ == "__main__":
    main()
