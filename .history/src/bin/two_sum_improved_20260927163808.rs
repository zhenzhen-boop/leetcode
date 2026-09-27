use std::collections::HashMap;


// ハッシュマップを利用している。
// 与えられたVec<i32>のnumsのインデックスの2つvecを返さなければならない。
// ハッシュマップには最初からすべてのnumsの要素を入れてはいけない（[5,7,3,23,3]みたいなnumsがあった時に3のインデックスがハッシュマップでは上書きされてしまう。）
// numsのnumのインデックスを1つずつ見ていき、complement = target - numのものを探していき、

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map = HashMap::new();

        let mut ans_vec:Vec<i32> = Vec::new();

        for idx in 0..nums.len(){
            let complement = target - nums[idx];

            if map.contains_key(&complement){
                ans_vec.push(map[&complement]);
                ans_vec.push(idx as i32);
                return ans_vec;
            }

            map.insert(nums[idx],idx as i32);
        }
        return ans_vec;

    }
}


fn main(){
    let ans = Solution::two_sum(vec![2,7,11,15],9);

    println!("{:?}",ans);
}