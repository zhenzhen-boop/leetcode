class Solution:
    def longestPalindrome(self, s: str) -> str:
        max_odd_cnt = 0
        max_substring_of_odd_len = ""
        
        if len(s) != 0:
            max_substring_of_odd_len = s[0]

        for idx in range(len(s)):
            left = idx 
            right = idx 
            odd_moving_cnt = 0

            while left > 0 and right < len(s)-1 :
                if  s[left-1] == s[right+1]:
                    left -= 1
                    right += 1
                    odd_moving_cnt += 1    
                else:
                    break    
                #print(f"idx : {idx} , moving_cnt : {moving_cnt}")

            if max_odd_cnt < odd_moving_cnt:
                #print(f"max_substring : {max_substring}")
                max_odd_cnt = odd_moving_cnt
                
                
                max_substring_of_odd_len = s[left:right+1]
                    #print(max_substring)       
                           
        max_even_cnt = 0
        
        for idx in range(1,len(s)-1):
            left = idx
            right = idx+1
            max_substring_of_even_len = ""
            
            while left > 0 and right < len(s)-1 and s[left-1] == s[right+1]:
                left -= 1
                right += 1
                
            if max_even_cnt < right - left + 1:
                max_even_cnt = right - left + 1
                max_substring_of_even_len = s[left:right+1]
                
        if len(max_substring_of_even_len) > len(max_substring_of_odd_len):
            print("even!")
            max_substring = max_substring_of_even_len
        
        else:
            print("odd!")
            max_substring = max_substring_of_odd_len                  
            
        return max_substring
    
def main():
    print(Solution().longestPalindrome("babad"))
    #print(Solution().longestPalindrome("cbbd"))
    #print(Solution().longestPalindrome("a"))
    #print(Solution().longestPalindrome("aa"))
    print(Solution().longestPalindrome("ccd"))
    print(Solution().longestPalindrome("aacabdkacaa"))
    


if __name__ == "__main__":
    main()    