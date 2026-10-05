# Definition for singly-linked list.
class ListNode:
   def __init__(self, val=0, next=None):
       self.val = val
       self.next = next

class Solution:
    def mergeTwoLists(self, list1: ListNode | None, list2: ListNode | None) -> ListNode | None:
        # 両方Noneだったら...??

        if list1 == None:
            return list2
        if list2 == None:
            return list1

        
        ptr1 = list1
        ptr2 = list2
        merged_ptr = None
        
        if ptr1.val > ptr2.val:
            merged_ptr = ptr2
            ptr1 = ptr1.next
            
        else:
            merged_ptr = ptr1
            ptr2 = ptr2.next    
             
        while ptr1 != None and ptr2 != None:
            if ptr1.val > ptr2.val:
                merged_ptr.next = ptr2
                ptr1 = ptr1.next
                        
            else:
                merged_ptr.next = ptr1
                ptr2 = ptr2.next  
            
            
            