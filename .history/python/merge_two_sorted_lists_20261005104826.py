# Definition for singly-linked list.
class ListNode:
   def __init__(self, val=0, next=None):
       self.val = val
       self.next = next

class Solution:
    def mergeTwoLists(self, list1: ListNode | None, list2: ListNode | None) -> ListNode | None:
        # 両方Noneだったら...??
        # リストに慣れていないのが反省

        if list1 == None:
            return list2
        if list2 == None:
            return list1

        
        ptr1 = list1 # ここは共にNoneではない
        ptr2 = list2
        merged_ptr = None
        
        if ptr1.val > ptr2.val:
            merged_ptr = ptr2
            ptr2 = ptr2.next
            
        else:
            merged_ptr = ptr1
            ptr1 = ptr1.next 
            
        # ここでheadを定めても、merged_ptrの変更は反映される(?ポインタ?)    
        merged_list_head = merged_ptr # これもNoneではない
        #merged_ptr = merged_ptr.next       
             
        while ptr1 != None or ptr2 != None:
            #merged_ptr = merged_ptr.next # ここでmerged_ptrはNoneになるかもしれない

            if ptr2 == None:
                merged_ptr.next = ptr1
                if ptr1 != None:
                    ptr1 = ptr1.next

                        
            elif ptr1 == None :
                merged_ptr.next = ptr2
                if ptr2 != None:
                    ptr2 = ptr2.next 

            # ここからはptr1もptr2もNoneでない
            elif ptr1.val > ptr2.val:
                merged_ptr.next = ptr2
                if ptr2 != None:
                    ptr2 = ptr2.next 

            elif ptr2.val >= ptr1.val:
                merged_ptr.next = ptr1
                if ptr1 != None:
                    ptr1 = ptr1.next

            merged_ptr = merged_ptr.next # ここでmerged_ptrはNoneにならないはず
                
            
        return  merged_list_head
            
            
def main():
    print(f"{Solution().mergeTwoLists()}")            