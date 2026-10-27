struct Solution{}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {

        if num_rows == 1 || num_rows >= s.len() as i32{
            return s;
        }
        let num_rows_usize:usize = num_rows as usize;

        // ここの要素数の指定の仕方も覚えておく
        let mut arr :Vec<String> = vec!["".to_string();num_rows_usize]; 

        let mut step : usize = 2; // usizeはi32のように負の値は取れないから、idx = のところに-1を混ぜて調整する。だからstepは2スタートなことに注意
        let mut idx : usize = 0;
        

        for (_,ch) in s.chars().enumerate(){

            arr[idx] = arr[idx].clone() + &(ch.to_string()); // ここにclone()を入れなきゃいけないらしい

            //println!("{:?}",arr);

            if idx == num_rows_usize - 1 { // ここもusizeにするのを忘れずに
                step = 0;
            }

            else if idx == 0{
                step = 2; // ここも2にしないとダメ。注意
            }

            idx = idx + step -1 ;
        }

        // let ans = arr.concat(); // でもできるけど、join()だと区切る文字を指定できる
        let ans = arr.join("").to_string();

        return ans;

    }
}


fn main(){
    println!("{:?}",Solution::convert("kawaiidakeja,damedesuka".to_string(),3));
}