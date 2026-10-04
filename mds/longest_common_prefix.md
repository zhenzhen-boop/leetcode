# Intuition
<!-- Describe your first thoughts on how to solve this problem. -->

# Approach
<!-- Describe your approach to solving the problem. -->
与えられた配列の中で一番長さの短い文字列要素をmin_stringとする。min_stringの最初の文字から順に、その文字が配列の他要素の同じ指数部分に入っているかを確かめていく。その文字がすべての他要素の同指数部分入っていることが確かめられれば、max_prefixをそれまでに入っていると確かめられた文字列スライスに変更、入っていなければその時点でのmax_prefixを返す。foundはその文字が他要素に入っているか示すフラグである。

# Complexity
- Time complexity:
<!-- Add your time complexity here, e.g. $$O(n)$$ -->
与えられた配列内の最小長文字列の長さをm、配列長をnとするとO(mn)で与えられる。m,nのどちらがより計算量に寄与するかは配列による。

- Space complexity:
<!-- Add your space complexity here, e.g. $$O(n)$$ -->
O(n)(nは配列の大きさ)

# Code
```python3 []
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
```