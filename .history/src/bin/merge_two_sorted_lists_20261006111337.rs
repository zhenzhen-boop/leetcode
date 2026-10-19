/*
Rustにおける pub は、関数や構造体、モジュールなどの項目を外部からアクセス可能（パブリック）にするためのアクセス修飾子

// 例
pub struct MyStruct {
    pub public_field: i32,    // 外部からアクセス可能
    private_field: i32,       // 外部からアクセス不可
}

• デフォルト（何もつけない場合）: 非公開（プライベート）になり、同じモジュール内からしかアクセスできない
• pub をつける場合: どこからでも（モジュールの外や別のファイル・クレートから）アクセスできるようになる

構造体に pub struct とつけても、構造体のフィールドはデフォルトで非公開（プライベート）のまま
フィールドも外部から直接読み書きできるようにするには、フィールドごとに pub をつける必要がある

*/

struct Solution{}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>> // これの意味は 
   // Optionにしておけば、値がないときにNoneを取ることができる。
   // Boxについて、普通は値はスタック領域に積まれてスタック内での積み直しなどによりアドレスが変わったりするが
   // Box<型名>というふうにしておけば、値はヒープ領域に確保されるからそのようなことが起こりにくい。
   // Boxは<>の中身をヒープ領域に置き、そのヒープへの所有権を持つスマートポインタ

   /*
    next: Option<ListNode>
    とすると、
    ListNode
    └─ ListNode
       └─ ListNode
            └─ ListNode
                 └─ ...となってコンパイラがListNodeの型の大きさを決めることができない

    */
}
 
impl ListNode { // ListNodeっていう構造体にかけるメソッド
   #[inline]
   fn new(val: i32) -> Self { // 新しいリストノードを作成
     ListNode {
       next: None,
       val
     }
   }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1 == None{ // list1.is_none()でもいい
            return list2 // return Some(list2) はおかしい、なぜならlist2はすでにOption型だから
        }
        if list2 == None{
            return list1
        }

        let mut ptr1 = list1;
        let mut ptr2 = list2;
        let mut merged_ptr : Option<Box<ListNode>> = None;

        // dummyというリストノードを作り、tailという可変参照でいじる
        // 返すのはdummy.next
        let mut dummy = ListNode::new(0);
        // これならptr1,2みたいにOption<Box<ListNode>>型じゃないからListNodeに直接アクセスできる
        let mut tail = &mut dummy; // taile.next == (*tail).nextになる


        // ptr1.unwrap().val > ptr2.unwrap().val は、ptr1、2の所有権を奪ってしまう。(ptr1、2がもう使えなくなってしまう)
        // as_ref()を使うと所有権を奪うことなくunwrap()ができる
        if ptr1.as_ref().unwrap().val > ptr2.as_ref().unwrap().val{
            tail.next = 
            // merged_ptr = ptr2; // これも所有権を持っていってしまう。
            // ptr2 = ptr2.next; 
            // これをすると、ptr2.nextの、ptr2への所有権の移動が起こる。
            // ptr2.next==??になってしまう。ptr1のフィールドの値を勝手に抜き取ってしまう。これをrustのコンパイラは許してくれない。
            // take()を使えばptr2.next==Noneにして所有権を移す。
            // take()は所有権を奪う代わりにNoneを残していく
            // 所有権を次のノードへ移していく

            /*
            元：
            ptr1
            ↓
            [1] → [2] → [3] → None

            take()

            ptr1 (ptr1を次のノードへ進めた)
            ↓
            [2] → [3] → None

            元の1のnextは、

            [1] → None  になる

            
             */

        }

        else{
            //merged_ptr = ptr1;
            ptr1 = ptr1.as_ref().unwrap().next.take();
        }

    }
}


fn main(){
    Solution::merge_two_lists();
}