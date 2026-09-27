use std::collections::HashMap;
use std::char;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {

        // 文字とその値の対応表を作る
        let symbol_value_map = HashMap::from(
            [('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)]
        );

        // 加算結果を表す変数
        let mut ans = 0;

        // 見ている文字の一個前の文字を保持する。
        let mut prev_character : char = 'a';

        for character in s.chars(){
            println!("{character}:{symbol_value_map[&character]}");
            //println!("{character}:{}",symbol_value_map[&character]);
            if prev_character == 'I'{
                if character == 'V'{
                    ans = ans - symbol_value_map[&prev_character] + 5;
                }
                else if character == 'X'{
                    ans = ans - symbol_value_map[&prev_character] + 4;
                }
            }

            else if prev_character == 'X'{
                if character == 'L'{
                    ans = ans - symbol_value_map[&prev_character] + 40;
                }
                else if character == 'C'{
                    ans = ans - symbol_value_map[&prev_character] + 90;
                }   
            }
            
            else if prev_character == 'C'{
                if character == 'D' {
                    ans = ans - symbol_value_map[&prev_character] + 400;
                }
                else if character == 'M'{
                    ans = ans - symbol_value_map[&prev_character] + 900;
                }
            }
            else{
                ans += symbol_value_map[&character];
            }
            prev_character = character;
        }

        return ans;
    }
}

fn main(){
    let sentence = "MCMXCIV".to_string();
    println!("{}",Solution::roman_to_int(sentence));
}