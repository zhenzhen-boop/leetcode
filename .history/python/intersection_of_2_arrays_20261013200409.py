class Solution(object):
    def intersection(self, nums1, nums2):
        p1 = set(nums1)
        p2 = set(nums2)

        return list(p1 & p2)    

        """
        :type nums1: List[int]
        :type nums2: List[int]
        :rtype: List[int]
        """
        