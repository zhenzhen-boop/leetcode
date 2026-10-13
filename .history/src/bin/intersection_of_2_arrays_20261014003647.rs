struct Solution{};

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        // 今回はi32型のベクターだけど、
        // ベクターの型指定をしたくなかったらVec:new()とすればいい
        let mut ans_list = Vec::new();
        
        if nums1.len() < nums2.len(){
            for idx in (0..nums1.len()){
                if nums2.contains(&nums1[idx]) && !ans_list.contains(&nums1[idx]){
                    ans_list.push(nums1[idx]); 
                    // appendではなくpushを使う
                    // pushは所有権を移動するから&はつけない　
                }
            }
        }
        
        else{
            for idx in (0..nums2.len()){
                if nums1.contains(&nums2[idx]) && !ans_list.contains(&nums2[idx]){
                    ans_list.push(nums2[idx]);
                }
            }
        }
        
        return ans_list;
        
        
    }
}
fn main(){

    let num1 = vec![1,2,3];
    let num2 = vec![6,5,4,3,2];

    println!("{}",Solution::intersection(nums1, nums2));
}