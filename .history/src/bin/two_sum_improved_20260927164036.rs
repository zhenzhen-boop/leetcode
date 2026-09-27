use std::collections::HashMap;


// ハッシュマップを利用している。
// 与えられたVec<i32>のnumsのインデックスの2つvecを返さなければならない。
// ハッシュマップには最初からすべてのnumsの要素を入れてはいけない（[5,7,3,23,3]みたいなnumsがあった時に3のインデックスがハッシュマップでは上書きされてしまう。）
// numsのインデックスを1つずつ見ていき、complement = target - nums[idx]を定義。
// complementがmapになかった時は(nums[idx],idx)をmapに追加していく。
// complementがすでにmapにあればmap[complement]、idxを返す。
// こうすることでメモリ効率や容量の良さは改善前より下がるが、スピードは抜群に上がる(O(n)だから)

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