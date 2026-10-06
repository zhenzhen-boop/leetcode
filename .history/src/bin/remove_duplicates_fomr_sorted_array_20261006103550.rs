struct Solution{}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx:usize = 1;
        
        while nums.len() != 0 && nums.len() != 1 && idx  < len(nums){
            if nums[idx] == nums[idx-1]{
                nums.remove(idx);
                if idx >= 0{
                    idx -= 1;
                }
            }

            else{
                idx += 1;
            }
            if idx == len(nums)-1 and nums[idx] != nums[idx-1]{
                break;
            }
        }
        return len(nums);
    }
}