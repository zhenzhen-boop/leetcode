class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        ##count = 1
        max_count = 1

        for slow_ptr in s:
            dic.append(slow_ptr)

            for fast_ptr in s[1:]:
                if fast_ptr not in dic:
                    ##count += 1
                    dic.append(fast_ptr)
                else:
                    if max_count < len(dic):##count:
                        max_count = len(dic)
                    ##count = 1
                    print(dic)
                    dic = []  
        return max_count  
    
    
print(Solution().lengthOfLongestSubstring("bbbbb"))    

