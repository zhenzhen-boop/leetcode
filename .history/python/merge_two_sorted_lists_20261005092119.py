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
            
        merged_list_head = merged_ptr
        merged_ptr = merged_ptr.next       
             
        while ptr1 != None or ptr2 != None:
            if ptr2 == None or ptr1.val > ptr2.val:
                merged_ptr = ptr2
                ptr1 = ptr1.next
                        
            elif ptr1 == None or ptr2.val > ptr1.val:
                merged_ptr = ptr1
                ptr2 = ptr2.next 
            
            merged_ptr = merged_ptr.next     
            
        return  merged_list_head
            