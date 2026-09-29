fn length_of_longest_substring(s:&str) -> i32 {

    let mut char_list :Vec<char> = Vec::new();
    let mut max_count  = 0;

    //let sliced_s = &s[0..]; // 意味ない

    // &strはインデックスアクセスできないから、char()にしてenumerateするのが一番いい。
    // あとはchars().nth().unwrap()とか(でもこれはO(n)かかるらしい)

    // idxはあるけど使わないから先頭にワイルドカードを付けておくといい。
    for (_idx,value )in s.chars().enumerate(){
        if !char_list.contains(&value){
            char_list.push(value);
        }

        else{
            while char_list.contains(&value){ // ここでは&valueなの何？
                char_list.remove(0);
            }
            char_list.push(value); // ここではvalueなのに-> pushはvalueの値自体の所有権をchar_listに渡すから
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


/*
https://zenn.dev/ekusiadadus/articles/rust-string-index

fn main() {
    let s = "こんにちは";

    // 方法1: .chars().nth() - O(n)の時間複雑度
    let third_char = s.chars().nth(2).unwrap(); // "に"
    println!("3番目の文字: {}", third_char);

    // 方法2: .chars()からのイテレーション - 全文字に順次アクセスする場合は効率的
    for (i, c) in s.chars().enumerate() {
        println!("文字 {}: {}", i, c);
    }

    // 方法3: バイトレベルでアクセス - 低レベル操作が必要な場合
    for b in s.bytes() {
        println!("バイト: {}", b);
    }

    // 文字単位でのスライス（非効率だが安全）
    let char_slice: String = s.chars().skip(1).take(2).collect();
    println!("文字スライス: {}", char_slice); // "んに"
}
*/