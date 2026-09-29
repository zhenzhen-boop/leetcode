fn length_of_longest_substring(s:&str) {

    let mut char_list :Vec<char> = Vec::new();

    for idx in 0..s.len(){
        if char_list.contains(s[idx]){
            char_list.push(s[idx]);
        }

        else{
            while s[idx] in char_list{
                char_list.remove(0);
            }
            char_list.insert()
        }
    }
}