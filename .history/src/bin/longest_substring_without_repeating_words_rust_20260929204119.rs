fn length_of_longest_substring(s:&str) -> i32{

    let mut char_list :Vec<char> = Vec::new();

    for idx in 0..s.len(){
        if char_list.contains(s[idx]){
            char_list.insert(0,s[idx]);
        }
    }
}