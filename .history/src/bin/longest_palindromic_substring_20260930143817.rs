fn main(){

}

struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        // char_listを作ってしまった方が簡単で見やすいかもしれない
        let char_list : Vec<char> = s.chars().collect();

        let mut max_odd_cnt = 0;
        let mut max_substring_of_odd_len :Vec<char>;

        if !s.is_empty(){
            max_substring_of_odd_len = char_list[0];
        }
        else{
            return ""
        }


        for idx in 0..s.len(){
            let mut left = idx;
            let mut right = idx;
            let mut odd_moving_cnt = 0;
            

            while left > 0 && right < s.len()-1{
                if char_list[left-1] == char_list[right+1]{
                    left -= 1;
                    right += 1;
                    odd_moving_cnt += 1;
                }
                else{
                    break;
                }    
            }

            if max_odd_cnt < odd_moving_cnt {
                max_odd_cnt = odd_moving_cnt;
                max_substring_of_odd_len = char_list[left:right+1];
            }
        }
        
        return ""
    }
}