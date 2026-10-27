// RustでHeapを使うには、std::collectionsの中にあるBinaryHeapを使う
use std::collections::BinaryHeap; 
// これはBinaryHeapが通常は最大ヒープなのに対して最小ヒープを実装するために用いる。
use core::cmp::Reverse;

struct KthLargest{
    k:usize,
    heap:BinaryHeap<Reverse<i32>> 
}

impl KthLargest{

    fn new(k:i32,nums:Vec<i32>)-> Self{
        Self {
            k : k as usize, // k番目のものを取り出すとしている。
            heap : BinaryHeap::from(Reverse(nums)),
        }
    }

    fn add(&mut self,val:i32) -> i32{

        // heapに値をpush
        self.heap.push(Reverse(val));

        // もしheapの長さがself.kより長ければ、heapのrootノードをさよなら
        if self.heap.len() > self.k {
            self.heap.pop();
        }

        return self.heap.peek().unwrap().0;


    }
}