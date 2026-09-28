class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        max_count = 0

        for slow_idx in range(len(s)-1):
            dic = []
            dic.append(s[slow_idx])
            max_count = len(dic)
            
            for fast_idx in range(slow_idx,len(s)):
                if s[fast_idx] not in dic:
                    dic.append(s[fast_idx])
                
                else:
                    if max_count < len(dic):
                        max_count = len(dic)   
                        #print(dic)
                        break


        return max_count
    
print(Solution().lengthOfLongestSubstring("S"))    

