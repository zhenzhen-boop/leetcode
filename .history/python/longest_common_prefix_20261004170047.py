class Solution:
    def longestCommonPrefix(self, strs: list[str]) -> str:
        min_string = min(strs,key=lambda x:len(x))
        FOUND : 1
        max_prefix = ""
        
        for idx in range(len(min_string)):
            if FOUND == 0:
                break
            for string in strs:
                if string[idx] != min_string[idx]:
                    FOUND = 0
                    break
            if FOUND == 1:
                max_prefix = min_string[0:idx+1]
                
        return max_prefix        
                
                
                
def main():
    
if __name__ == "__main__":
    main()                    