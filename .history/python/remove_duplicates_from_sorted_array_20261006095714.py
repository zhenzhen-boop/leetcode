class Solution:
# 返り値は何？型は？
    def removeDuplicates(self, nums: list[int]) -> int:
        #prev = nums[0]
        idx = 1
        nums_len = len(nums)
        

        while len(nums) != 0 and len(nums) != 1 and idx < len(nums):
            print(idx)
            if nums[idx] == nums[idx-1]:
                del nums[idx]
                idx -= 1
                
            else:
                idx += 1    
            
            if idx == len(nums)-1 and nums[idx] != nums[idx-1]: # これが最後の要素であるとき
                break      
                
        
        return len[nums]        


def main():
    print(Solution().removeDuplicates([0,1]))

if __name__ == "__main__":
    main()        