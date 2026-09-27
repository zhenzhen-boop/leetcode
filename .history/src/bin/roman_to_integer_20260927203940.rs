use std::collections::HashMap;
use std char;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {

        // 文字とその値の対応表を作る
        let symbol_value_map = HashMap::from(
            [('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)]
        );

        // 加算結果を表す変数
        let mut ans = 0;

        let prev_char = ;

        for character in s.chars(){
            println!("{character}:{}",symbol_value_map[&character]);
            ans += symbol_value_map[&character];
        }

        return ans;
    }
}

fn main(){
    let sentence = "MCMXCIV".to_string();
    println!("{}",Solution::roman_to_int(sentence));
}