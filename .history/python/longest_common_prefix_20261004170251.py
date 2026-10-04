class Solution:
    def longestCommonPrefix(self, strs: list[str]) -> str:
        min_string = min(strs,key=lambda x:len(x))
        found = 1
        max_prefix = ""
        
        for idx in range(len(min_string)):
            if found == 0:
                break
            for string in strs:
                if string[idx] != min_string[idx]:
                    found = 0
                    break
            if found == 1:
                max_prefix = min_string[0:idx+1]
                
        return max_prefix        
                
                
                
def main():
    print(f"{Solution().longestCommonPrefix(["a","ab"])}")
    
if __name__ == "__main__":
    main()                    