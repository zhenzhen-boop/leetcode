struct Solution;



impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //let mut left = 0;
        //let mut right = 1;
        let mut ans_vec:Vec<i32> = Vec::new();

        let mut nums_order = nums;

        nums_order.sort();

        for left in 0..nums_order.len()-1{
            for right in 1..nums_order.len(){
                let two_sum = nums_order[left] + nums_order[right];
                if two_sum == target{
                    // leftとrightはusize型になるからあとでi32に変換しなきゃいけない。 
                    ans_vec.push(left as i32); // ここで型の変換が必要。
                    ans_vec.push(right as i32);
                    return ans_vec;
                }

                if nums_order[left] + nums_order[right] > target && ans_vec.is_empty(){
                    return ans_vec;
            }

            }
            return ans_vec;
        }
        return ans_vec;
    }
}