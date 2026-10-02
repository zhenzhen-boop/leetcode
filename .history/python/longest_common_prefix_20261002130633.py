class Solution:
    def longestCommonPrefix(self, strs: list[str]) -> str:
        min_element = min(strs,key = lambda x: len[x])
        max_common_str = "" 
        idx = 0

        for left_idx in range(0,len(min_element)-1):
            idx = 0
            for right_idx in range(left_idx,len(min_element)):
                idx = 0
                while idx < len(strs) and min_element[left_idx:right_idx] in strs[idx]:
                    idx += 1
                if idx == len(strs) and len(min_element[left_idx:right_idx]) > len(max_common_str):
                    max_common_str = min_element[left_idx:right_idx]

            
        