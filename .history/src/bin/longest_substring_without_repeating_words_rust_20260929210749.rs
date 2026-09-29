fn length_of_longest_substring(s:&str) -> i32 {

    let mut char_list :Vec<char> = Vec::new();
    let mut max_count  = 0;

    //let sliced_s = &s[0..]; // 意味ない

    // &strはインデックスアクセスできないから、char()にしてenumerateするのが一番いい。
    // あとはchars().nth().unwrap()とか

    for (idx,value )in s.chars().enumerate(){
        if char_list.contains(&value){
            char_list.push(value);
        }

        else{
            while char_list.contains(&value){ // ここでは&valueなの何？
                char_list.remove(0);
            }
            char_list.push(value); // ここではvalueなのに
        }

        if max_count < char_list.len(){ // char_list_len()はusizeだけどmax_count はi32->max_countをusizeにしておいて、返り値だけi32にした
            max_count = char_list.len();
        }
    }

    return max_count as i32; // ここをi32にしないと返り値の型エラー
}


fn main(){
    println!("{}",length_of_longest_substring("pwwkew"));
}