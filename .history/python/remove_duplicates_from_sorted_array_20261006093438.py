class Solution:
# 返り値は何？型は？
    def removeDuplicates(self, nums: list[int]) -> int:
        prev = nums[0]
        for idx in range(1,len(nums)):
            if nums[idx] == prev:
                del nums[idx]
        
        return len[nums]        


def main():
    print(Solution().removeDuplicates([1,1,2,3,4,5,5]))

if __name__ == "__main__":
    main()        