struct Solution{}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        
        let mut idx : usize = 0;
        // let zero_cnt :i32 = nums.count(0); countメソッドは使えない
        let mut zero_cnt :usize = 0; 
        
        for cnt in (0..nums.len()){
            if nums[cnt] == 0{
                zero_cnt += 1;
            }
        }
        
        //println!("{zero_cnt}");
        
        
        while  nums.len() - zero_cnt > idx{ // このzero_cntがないと無限ループに陥る、zero_cntもusizeじゃないとだめ
            // ここに=を入れたら無限になるのはなぜだ？
            
            if nums[idx] == 0{
                nums.remove(idx); // 配列の要素を削除するメソッドはidxで指定するしかない
                // でもremove()はO(N)かかるから遅い　
                nums.push(0);
               // idx -= 1; // idxを一つ下げる必要ない
            }
            else{
                idx += 1;
            }
        }
        // println!("{idx}");
        return;
    }
}


fn main(){
    println!("{}",Solution::move_zeroes(vec![1,2,3]));
}