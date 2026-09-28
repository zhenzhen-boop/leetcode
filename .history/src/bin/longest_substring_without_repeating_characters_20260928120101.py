class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        count = 0
        max_count = 0

        for slow_ptr in s:
            dic.append(slow_ptr)

            for fast_ptr in s[1:]:
                if fast_ptr not in dic:
                    count += 1
                    dic.append(fast_ptr)
                else:
                    if max_count < count:
                        max_count = count 
                    count = 0
                    dic = []  
        return max_count  
    
    
print(Solution.lengthOfLongestSubstring("b"))    