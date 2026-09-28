class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        max_count = 0
        
        if len(s) <= 1: ## これがないと、sが０、１の時にfor文に入らない
            return len(s)

        for slow_idx in range(len(s)-1): ## このままだとlen(s) == 1の時にfor文に入らない
            dic = []
            dic.append(s[slow_idx])
            
            for fast_idx in range(slow_idx+1,len(s)):
                #if s[fast_idx] not in dic:
                if s[fast_idx] not in set(dic):
                    dic.append(s[fast_idx])
                    print("for dic:",end="")
                    print(dic)
                    if max_count < len(dic):
                        max_count = len(dic)   
                        print(dic)
                        
                else:
                    if max_count < len(dic):
                        max_count = len(dic)  
                    break
            
        return max_count
    
##print(Solution().lengthOfLongestSubstring("abcabcbb"))   
 
## できたけど、これだとruntime overになる
# if fast_idx not in s[..]はO(n)かかるらしいから、これをなくせばいい
# やったけどダメだった


### sliding windowというやつがいいらしい

class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        dic = []
        max_count = 0
        

        for idx in range(len(s)): ## このままだとlen(s) == 1の時にfor文に入らない
            if s[idx] not in dic:
                dic.append(s[idx])
                print(f"dic1:{dic}")
            
            else:
                del dic[0]
                dic.append(s[idx])
                print(f"dic2:{dic}")
            
            if max_count < len(dic):
                max_count = len(dic)
        
        return max_count                
                              
print(Solution().lengthOfLongestSubstring("pwwkew"))   




