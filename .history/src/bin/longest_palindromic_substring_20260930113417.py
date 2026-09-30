class Solution:
    def longestPalindrome(self, s: str) -> str:
        max_moving_cnt = 0
        max_substring = ""

        for idx in range(1,len(s)-1):
            left = idx - 1
            right = idx + 1
            moving_cnt = 0

            while left >= 0 and right < len(s) and s[left] == s[right]:
                left -= 1
                right += 1
                moving_cnt += 1

            if max_moving_cnt < moving_cnt:
                max_substring = s[left:right+1]
                max_moving_cnt = moving_cnt

        return max_substring