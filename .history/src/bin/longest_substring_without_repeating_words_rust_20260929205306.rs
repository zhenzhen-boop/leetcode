fn length_of_longest_substring(s:&str) {

    let mut char_list :Vec<char> = Vec::new();

    let sliced_s = &s[0..];

    for idx in 0..sliced_s.len(){
        if char_list.contains(sliced_s[idx]){
            char_list.push(sliced_s[idx]);
        }

        else{
            while char_list.contains(sliced_s[idx]){
                char_list.remove(0);
            }
            char_list.insert();
        }
    }
}