fn main(){

}

struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        // char_listを作ってしまった方が簡単で見やすいかもしれない
        let char_list : Vec<char> = s.chars().collect();

        let mut max_odd_cnt = 0;
        let mut max_substring_of_odd_len :&str;

        if !s.is_empty(){
            max_substring_of_odd_len = std::str(char_list[0]);
        }
        else{
            return "";
        }

        let mut max_substring = String::new();


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
                max_substring_of_odd_len = &char_list[left..right+1].collect();
            }
        }

        let mut max_even_cnt = 0;
        let mut max_substring_of_even_len :&str;

        for idx in 0..char_list.len()-1{
            let mut left = idx;
            let mut right = idx + 1;

            if char_list[left] != char_list[right]{
                continue;
            }

            while left > 0 && right < char_list.len()-1 && char_list[left-1] == char_list[right+1]{
                left -= 1;
                right += 1;
            }

            if max_even_cnt < right - left + 1{
                max_even_cnt = right - left + 1;
                max_substring_of_even_len = char_list[left..right + 1].iter().collect();
            }
        }

        if max_substring_of_even_len.chars().count() > max_substring_of_odd_len.chars().count(){
            max_substring = max_substring_of_even_len.to_string();
        }
        else{
            max_substring = max_substring_of_odd_len.to_string();
        }        
        return max_substring;
    }
}