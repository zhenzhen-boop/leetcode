class Solution:
    def plusOne(self, digits: list[int]) -> list[int]:
        
        # ansはdigitsの中の数字を数値化したもの
        # [1,2,3] -> ans = 123みたいな
        ans = 0
        
        # 一度数値化してしまうのが1番簡単かなと思ったけれど、しなくてもできそうだな
        # 要素が9だった時に再帰的に1を足していく事ができそうだけど、どっちの方がメモリ、速度的にベターだろうか
        
        for digit in range(len(digits)):
            ans = ans + digits[digit] * (10**(len(digits)-digit-1)) 
            
        print(ans)
        
        # これをリスト化する    
        plused_ans = ans + 1
        
        # digits_str_arr = str(plused_ans).split("") # これで一文字ずつ分けるのはできないみたい
        # だからlist()でやっちゃう
        # 一度文字列に変換してやるのが1番簡単かもしれない
        
        digits_str_arr = list(str(plused_ans))
        
        # こうやってstr list をint listに変換する
        digits_int_arr = [int(c) for c in digits_str_arr]
        
        return digits_int_arr
        
        
        