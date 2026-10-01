struct Solution{}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // colect()メソッドは、新たなコレクションを作成してくれる
        let x_char_vec:Vec<char> = x.to_string().chars().collect();

        let mut left = 0;
        let mut right = x_char_vec.len()-1;

        for cnt in 0..x_char_vec.len(){

            if cnt > (x_char_vec.len() / 2){
                break;
            }
            if x_char_vec[left] == x_char_vec[right]{
                left = left + 1;
                right = right - 1;
                continue;
            }
            else{
                return false;
            }

            
        }
        return true;
    }
}

fn main(){
    println!("{}",Solution::is_palindrome(10022201));
}