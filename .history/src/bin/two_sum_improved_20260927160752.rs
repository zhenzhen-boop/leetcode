use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //let mut left = 0;
        //let mut right = 1;
        let mut map = HashMap::new();

        for idx in 0..nums.len() {
            map.insert(nums[idx],idx as i32);
        }

        let mut ans_vec:Vec<i32> = Vec::new();


        for key in map.keys(){
            let complement = target - key;

            if let Some(&ok) = map.get(c&omplement){
                ans_vec.push(map[key]);
                ans_vec.push(map[&complement]);
                return ans_vec;
            }
        }
        return ans_vec;
    }
}


fn main(){
    let ans = Solution::two_sum(vec![2,7,11,15],9);

    println!("{:?}",ans);
}