class Solution:
    def longestCommonPrefix(self, strs: list[str]) -> str:
        
        if len(strs) == 0:
            return ""
        
        min_element = min(strs,key = lambda x: len(x))
        #print(f"min_element:{min_element}")
        #print(f"len(strs):{len(strs)}")
        max_common_str = "" 
        idx = 0
        
        
        if len(strs) == 1:
            return min_element

        for left_idx in range(0,len(min_element)):
            idx = 0
            for right_idx in range(left_idx,len(min_element)):
                idx = 0
                while idx < len(strs) and min_element[left_idx:right_idx+1] in strs[idx]:
                    idx += 1
                if idx == len(strs) and len(min_element[left_idx:right_idx+1]) > len(max_common_str):
                    max_common_str = min_element[left_idx:right_idx+1]
        
        return max_common_str            

def main():
    print(Solution().longestCommonPrefix(["ab", "a"]))
    print(Solution().longestCommonPrefix(["flower","flow","flight"]))
            
        
if __name__ == "__main__"  :
    main()      