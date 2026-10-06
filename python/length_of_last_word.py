class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        # 空白で文字列を分けるのってどうやってやるんだっけ
        # 覚えておくこと
        
        strs = s.split(' ')
        
        # print(strs)
        #print(type(strs))
        
        # for文を降順で回したい時にはreversedを使う
        # らしいけど使えないからこの3つの引数を取るパターンで書こう。
        for idx in range(len(strs)-1,-1,-1):
            #print(idx)
            #print(strs[idx]== "")
            
            # これが空白文字でないのはなぜだ...strsが''になってるけど,,,
            if strs[idx] != "":
                return len(strs[idx])
        return 0