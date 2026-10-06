class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        
        for idx in range(0,len(haystack)):
            # hay_idx = idx はだめ
            hay_idx = 0 # hay_idxはidxを先頭とした時の文字列内の指数
            needle_idx = 0 # needleの文字列の指数
            # cnt = 0 #cntはneedle文字列とhaystack文字列がどこまで一致しているかをカウントするためのカウンタ
            # だけどこれもhay_idxがあればいらない
            
            while hay_idx < (len(haystack) - idx) and needle_idx < len(needle) and haystack[idx + hay_idx] == needle[needle_idx]:
                #print(f"hay_idx:{hay_idx}")
                #print(f"idx + hay_idx : {idx ; hay_idx}")
                hay_idx += 1
                needle_idx += 1
                #cnt += 1
            if hay_idx == len(needle): # ここは-1しなくていい、なぜならhay_idxは+1しているから終わる時にはlen(needle)と等しくなっているから
                return idx
         
        return -1                      
                