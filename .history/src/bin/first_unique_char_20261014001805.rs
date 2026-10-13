struct Solution{}

use std::collections::HashMap; // これを忘れない

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        
        // シンプルに辞書を作って出現回数を記録するか
        
        // 文字列中の文字の出現回数を記録していく
        let mut s_dictionary : HashMap<char,(i32,usize)> = HashMap::new();
        
        //let s = s.as_str();
        
        
        // rustは文字列のインデックス操作ができないからenumerateでやった方が絶対いい。
        // idxだけ取ってもs[idx]みたいにできないからだめ。
        // .nth()のメソッドはO(n)かかるからあかん
        
        for (idx,ch) in s.chars().enumerate(){
            if let Some(tuple) = s_dictionary.get_mut(&ch){
                tuple.0 += 1;
            } 
            else{
                s_dictionary.insert(ch,(1,idx));
            }
        }
        
        for character in s.chars(){ // 文字列はそのままfor文に投げることはできないからchars()を使う
            if s_dictionary[&character].0 == 1{
                return s_dictionary[&character].1 as i32;
            }
        }
        
        return -1;
    }
}

fn main(){
    println!("{}",Solution::first_uniq_char("inunu".to_string()));
}
        
        
        /*没
        let s = &s;
        
        for (idx,ch) in s.chars().enumerate(){
            // enumerate()ではidx,chの順番なことに気をつけること　
            // String型ってfor文の条件に投げられるんだっけ
            
           // if s_dictionary.contains(s[idx]){ // HashMapにはcontains()メソッドはないらしい
        // Some()を使う練習をする
        // 指定したキーが存在していれば
            if let Some(tuple) = s_dictionary.get_mut(&ch){ // ここの&はなんだ
                // get_mut()を使っているからtuple.0 += 1みたいな可変操作ができる
                tuple.0 += 1;
                // entryメソッドは、キーがすでに存在すればその値を書き換える
                // なければ新規挿入する
            }   
            
            else{
                s_dictionary.insert(ch,(1,idx)); //こうやってキーと値を追加する
                //これは追加順にキーと値が並ぶとは限らない
                
            }
        }
        
        // insert()は追加順に値が並ぶわけではないから、ソートしよう
        
        // sort_byを使うと思うが、わかりづらい　
        //s_dictionary.sort_by_key(|a| a.1.1);
        // これはそもそもだめ、HashMapにはsortメソッドがそもそもない
        
        println!("{:?}",s_dictionary);
        
        for element in &s_dictionary{ 
            // このs_dictionaryに&をつけなきゃいけないのはなぜ？
            // 所有権の移動がみたいな感じのことを言っている
            
            if let Some(tuple) = s_dictionary.get(&element.0){ // ここで&をつける理由は?
                if tuple.0 == 1{ //もし出現回数が1だったら
                    return tuple.1 as i32;
                }
            }
        }
        return -1;
        */
        
        /* 以下没関数 -> これはちょっとメモリ消費量が多そうでめんどくさい
        // 文字列sをアルファベット順（何順でもいい）で並べ替えて前後に同じ要素があるかで判定
        
        
        // 文字列の中身を並べ替えるには一度Vec<char>にしなきゃいけない,Stringのままではできない
        let mut sorted_s_list: Vec<char> = s.chars().collect();
        sorted_s_list.sort();
        
        // 文字列をリスト化するのってどうやるんだっけ
        // -> collect()メソッドを使うんだ
        let mut s_list : Vec<char>= s.chars().collect(); // これでVecになるのかな
        
        // これs.len() == 0の時、for文エラー出ないかな？
        // -> if文追加しておく？
        
        if s.len() == 0{
            return -1;
        }
        
        for idx in (0..s.len()-1){
            if sorted_s_list[idx] == sorted_s_list[idx+1]{
                continue;
            }
            else{
                s_list.pop(idx);// ここ
            }
        }
        
        for idx in (0..sorted_s_list.len()){
            if s_list.contains(&sorted_s_list[idx]){ // &をつけなきゃいけないらしい
                return idx as i32; // 忘れがち
            }
            else{
                continue;
            }
        }
        return -1; // 見つからなかった
        */


/*
let s = "こんにちは";
    println!("バイト数: {}", s.len()); // 15バイト（各日本語文字は3バイト）
    println!("文字数: {}", s.chars().count()); // 5文字
*/