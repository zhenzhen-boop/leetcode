struct Solution{}

// pythonのやつをそのまま書き写しただけだとできない -> idxはusizeで負の値を取ることができない

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx:usize = 1;
        
        while nums.len() != 0 && nums.len() != 1 && idx  < nums.len(){
            if idx == 0{
                idx += 1;
            }

            if  nums[idx] == nums[idx-1]{
                nums.remove(idx);
                if idx > 0{
                    idx -= 1; 
                }
            }

            else{
                idx += 1;
            }
            if idx == nums.len()-1 && nums[idx] != nums[idx-1]{
                break;
            }
        }
        return nums.len() as i32; // len()メソッドはどうやらusizeを返すらしい
    }
}

fn main(){
    println!("{}",Solution::remove_duplicates(&mut [1,1,2,2,3,3].to_vec()))
}