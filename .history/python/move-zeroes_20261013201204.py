class Solution: ### こっちの方が速いらしい
    def moveZeroes(self, nums) :
        slow = 0
        for fast in range(len(nums)):
            if nums[fast] != 0 and nums[slow] == 0:
                nums[slow], nums[fast] = nums[fast], nums[slow]

            # wait while we find a non-zero element to
            # swap with you
            if nums[slow] != 0:
                slow += 1

        """
        :type nums: List[int]
        :rtype: None Do not return anything, modify nums in-place instead.
        """
        