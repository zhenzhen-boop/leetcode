use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map = HashMap::new();

        let mut ans_vec:Vec<i32> = Vec::new();

        for num in 0..nums.len(){
            let complement = target - nums[num];

            if map.contains_key(&complement){
                ans_vec.push(map[&complement]);
                ans_vec.push(num as i32);
                return ans_vec;
            }

            map.insert(num,nums[num]);
        }
        return ans_vec;

    }
}


fn main(){
    let ans = Solution::two_sum(vec![2,7,11,15],9);

    println!("{:?}",ans);
}