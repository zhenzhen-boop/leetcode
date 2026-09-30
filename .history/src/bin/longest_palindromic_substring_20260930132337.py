class Solution:
    def longestPalindrome(self, s: str) -> str:
        max_moving_cnt = 0
        max_substring = ""
        same_char_cnt = 1
        max_same_char_cnt = 0
        
        if len(s) != 0:
            max_substring = s[0]

        for idx in range(len(s)):
            left = idx 
            right = idx 
            moving_cnt = 0
            
            
            if idx > 0 :
                if s[idx-1] == s[idx]:
                    same_char_cnt += 1
                    if same_char_cnt > max_same_char_cnt:
                        max_same_char_cnt = same_char_cnt  
                                            #same_char_cnt = 1   
                        max_same_char_start_idx = idx - max_same_char_cnt
            else:
                same_char_cnt = 1
                
                            
            
            print(f"max_same_char_cnt : {max_same_char_cnt}")

            while left > 0 and right < len(s)-1 :
                if  s[left-1] == s[right+1]:
                    left -= 1
                    right += 1
                    moving_cnt += 1    
                else:
                    break    
                print(f"idx : {idx} , moving_cnt : {moving_cnt}")

            if max_moving_cnt < moving_cnt:
                #print(f"max_substring : {max_substring}")
                max_moving_cnt = moving_cnt
            print(f"idx : {idx} , max_moving_cnt : {max_moving_cnt}")
                
                if max_moving_cnt*2 + 1 > max_same_char_cnt:
                    max_substring = s[left:right+1]
                    #print(max_substring)
                
                else:
                    max_substring = s[max_same_char_start_idx:max_same_char_cnt+1]        
                        
            else:
                if max_moving_cnt*2 + 1 < max_same_char_cnt:
                    max_substring =  s[max_same_char_start_idx:max_same_char_cnt+1]  
                            
        return max_substring
    
def main():
    #print(Solution().longestPalindrome("babad"))
    #print(Solution().longestPalindrome("cbbd"))
    #print(Solution().longestPalindrome("a"))
    print(Solution().longestPalindrome("aa"))
    


if __name__ == "__main__":
    main()    