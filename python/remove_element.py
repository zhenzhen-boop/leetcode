class Solution:
    def removeElement(self, nums: list[int], val: int) -> int:
        idx = 0
        nums_len = len(nums)
        
        while idx <  nums_len:
            
            ## 削除する要素が見つかった
            if nums[idx] == val:
                del nums[idx]
                # idx -= 1 # これはいらない
                nums_len = len(nums) # 配列の大きさを更新
                
            else:
                idx += 1    
                