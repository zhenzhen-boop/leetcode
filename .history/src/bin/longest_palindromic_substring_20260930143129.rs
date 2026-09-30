fn main(){

}

struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max_odd_cnt = 0;
        let mut max_substring_of_odd_len : String = String::new();

        if !s.is_empty(){
            max_substring_of_odd_len = s.chars().nth(0).unwrap().to_string();
        }
        else{
            return "";
        }

        for idx in 0..s.len(){
            let mut left = idx;
            let mut right = idx;
            let mut odd_moving_cnt = 0; 

            while left > 0 and right < s.len()-1{
                if 
            }
        }
        

    }
}