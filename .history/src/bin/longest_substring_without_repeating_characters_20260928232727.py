class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        max_count = 0
        
        if len(s) <= 1: ## これがないと、sが
            return len(s)

        for slow_idx in range(len(s)-1): ## このままだとlen(s) == 1の時にfor文に入らない
            dic = []
            dic.append(s[slow_idx])
            
            for fast_idx in range(slow_idx+1,len(s)):
                if s[fast_idx] not in dic:
                    dic.append(s[fast_idx])
                    print("for dic")
                    print(dic)
                
                else:
                    if max_count < len(dic):
                        max_count = len(dic)   
                        print(dic)
                        break


        return max_count
    
print(Solution().lengthOfLongestSubstring("mq"))    

