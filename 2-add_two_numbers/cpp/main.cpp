#include <iostream>

/* Definition for singly-linked list. */
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
  void printLinkedList(ListNode *node) {
    while (node != nullptr) {
      std::cout << node->val << " ";
      node = node->next;
    }
    std::cout << "\n";
  }

  ListNode *reverseLinkedList(ListNode *node) {
    ListNode *curr = node;
    ListNode *prev = nullptr;
    ListNode *nextNode;

    std::cout << "Values: ";
    while (curr != nullptr) {
      /* std::cout << curr->val << "\n"; */
      std::cout << (curr->val) << " ";
      nextNode = curr->next;
      curr->next = prev;

      prev = curr;

      curr = nextNode;
    }
    std::cout << "\n";

    return prev;
  }

  ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
    /* l1 = this->reverseLinkedList(l1); */
    /* l2 = this->reverseLinkedList(l2); */

    ListNode *result = new ListNode();
    ListNode *p = result;
    unsigned short carry = 0;
    unsigned short val = 0;

    while (l1 != nullptr || l2 != nullptr || carry) {
      /* std::cout << l1->val << " " << l2->val << "\n"; */

      unsigned short sum = 0;

      if (l1 != nullptr) {
        sum += l1->val;
        l1 = l1->next;
      }

      if (l2 != nullptr) {
        sum += l2->val;
        l2 = l2->next;
      }

      sum += carry;
      carry = sum / 10;

      p->next = new ListNode(sum % 10);
      p = p-> next;
    }

    /* this->printLinkedList(result); */
    return result -> next;
  }
};

int main() {
  ListNode *l1 = new ListNode(0);
  /* ListNode *l2 = new ListNode(4, l1); */
  /* ListNode *l3 = new ListNode(2, l2); */

  ListNode *r1 = new ListNode(0);
  /* ListNode *r2 = new ListNode(6, r1); */
  /* ListNode *r3 = new ListNode(5, r2); */

  Solution s = Solution();
  s.printLinkedList(s.addTwoNumbers(l1, r1));


  return 0;
}
