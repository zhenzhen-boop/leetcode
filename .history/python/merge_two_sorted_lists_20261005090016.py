# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: ListNode | None, list2: ListNode | None) -> ListNode | None:
        # 両方Noneだったら...??

        if list1 == None:
            return list2
        if list2 == None:
            return list1

        
        ptr1_node = list1.val
        ptr2_node = list2.val
        merged_list_head = 

        while ptr1_node != None and ptr2_node != None:
