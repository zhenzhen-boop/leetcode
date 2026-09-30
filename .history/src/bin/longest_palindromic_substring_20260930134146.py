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


            while left > 0 and right < len(s)-1 :
                if  s[left-1] == s[right+1]:
                    left -= 1
                    right += 1
                    moving_cnt += 1    
                else:
                    break    
                #print(f"idx : {idx} , moving_cnt : {moving_cnt}")

            if max_moving_cnt < moving_cnt:
                #print(f"max_substring : {max_substring}")
                max_moving_cnt = moving_cnt
                
                
                max_substring = s[left:right+1]
                    #print(max_substring)       
                                   
              
        return max_substring
    
def main():
    print(Solution().longestPalindrome("babad"))
    #print(Solution().longestPalindrome("cbbd"))
    #print(Solution().longestPalindrome("a"))
    #print(Solution().longestPalindrome("aa"))
    #print(Solution().longestPalindrome("ccd"))
    #print(Solution().longestPalindrome("aacabdkacaa"))
    


if __name__ == "__main__":
    main()    