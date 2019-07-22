#include <iostream>
#include <stack>

using namespace ::std;

struct ListNode {
    int value;
    ListNode *next;
};

void print_list_reverse1(ListNode *pHead) {
    stack<ListNode *> a;
    auto p = pHead;
    while (p != nullptr) {
        a.push(p);
        p = p->next;
    }

    while (!a.empty()) {
        p = a.top();
        cout << p->value;
        if (a.size() != 1) {
            cout << "->";
        }
        a.pop();
    }
}


void print_list_reverse2(ListNode *pHead) {
    if (pHead != nullptr) {
        if (pHead->next != nullptr) {
            print_list_reverse2(pHead->next);
        }
        cout << pHead->value << "\t";
    }
}

int main() {
    auto a = ListNode{
            1,
            nullptr,
    };
    auto b = ListNode{
            2,
            nullptr,
    };
    auto c = ListNode{
            5,
            nullptr,
    };
    a.next = &b;
    b.next = &c;

    print_list_reverse2(&a);
    return 0;
}