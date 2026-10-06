class Solution:
# 返り値は何？型は？
    def removeDuplicates(self, nums: list[int]) -> int:
        #prev = nums[0]
        idx = 1
        nums_len = len(nums)
        

        while len(nums) != 0 and len(nums) != 1:
            if nums[idx] == nums[idx-1]:
                idx -= 1
                del nums[idx]
            
            if idx == len(nums)-1: # これが最後の要素であるとき
                break      
                
        
        return len[nums]        


def main():
    print(Solution().removeDuplicates([1,1,2,3,4,5,5]))

if __name__ == "__main__":
    main()        