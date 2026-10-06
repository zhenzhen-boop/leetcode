class Solution:
# 返り値は何？型は？
    def removeDuplicates(self, nums: list[int]) -> int:
        set_nums = set(nums)
        arr = []

        for num in set_nums:
            arr.append(num)

        arr = sorted(arr)

        for _ in range(len(arr)+1,len(nums)):
            arr.append(_)

        return arr