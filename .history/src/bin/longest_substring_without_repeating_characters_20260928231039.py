class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        ##count = 1
        max_count = 1

        for slow_ptr_idx in len(s):
            #dic.append(slow_ptr)

            for fast_ptr in s[slow_ptr_idx:]:
                if fast_ptr not in dic:
                    ##print(f"dec:{dic}")
                    ##count += 1
                    dic.append(fast_ptr)
                else:
                    if max_count < len(dic):##count:
                        max_count = len(dic)
                    ##count = 1
                    print(dic)
                    dic = []  
        return max_count  
    
    
print(Solution().lengthOfLongestSubstring("abcabcbb"))    

