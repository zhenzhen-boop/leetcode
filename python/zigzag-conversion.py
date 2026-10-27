class Solution:
    def convert(self, s: str, numRows: int) -> str:

        # 漸化式を立ててO(n^2)でやればできないことはないけれど、こちらの方が圧倒的に賢かった 
        # -> 単純に考えてやってはいけなかった
        if numRows == 1 or numRows >= len(s):
            return s

        arr = [""] * numRows
        step = 1
        idx = 0

        for ch in s:
            arr[idx] = arr[idx] + ch

            if idx == numRows-1:
                step = -1

            elif idx == 0:
                step = 1

            idx = idx + step        

        ans = "".join(arr)

        return ans
        
        