impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = 1;
        let mut ans_vec:Vec<i32> = Vec::new();

        let mut nums_order = nums;

        nums_order.sort();

        for left in range(0..nums_order.len()-1){

            if nums_order[left] + nums_order[right] > target && ans_vec.is_empty(){
                return ans_vec;
            }

            for right in range (1..nums_order.len()){
                let two_sum = nums_order[left] + nums_order[right];
                if two_sum == target{
                    ans_vec.append(left);
                    ans_vec.append(right);
                    return ans_vec;
                }
            }
            return ans_vec;
        }
    }
}