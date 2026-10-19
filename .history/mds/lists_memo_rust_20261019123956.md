

# Rust 学習ノート：Merge Two Sorted Lists

- テーマ：単方向連結リスト、所有権、借用、スマートポインタ
- 問題：2つの昇順ソート済み連結リストを1つの昇順リストにマージする
- 使用言語：Rust

---

Rust 学習ノート：Merge Two Sorted Lists
目次
1. pub とアクセス制御
2. ListNode の定義
3. なぜ Option<Box<ListNode>> なのか
4. ListNode::new() と Self
5. Rust の所有権（Ownership）
6. unwrap()、take()、as_ref()、as_mut()
7. node を作る理由
8. dummy と tail
9. while の条件：&& と ||
10. merge_two_lists の完成コード
11. main() でリストを作ってテストする
12. 学習時の重要な Tips
13. 最後に覚えておきたいこと

## 1\. `pub` とアクセス制御

Rustでは、`pub` をつけることで、関数・構造体・フィールドなどを外部から利用可能にできる。

```
pub struct MyStruct {
    pub public_field: i32,
    private_field: i32,
}
```

- `pub struct MyStruct`：構造体を公開する。
- `pub public_field`：フィールドを公開する。
- `private_field`：フィールドを公開しない。

**注意：** `pub struct` と書いても、フィールドまで自動的に公開されるわけではない。

また、非公開項目へのアクセス範囲は、ファイル単位ではなくモジュール構造によって決まる。

---

## 2\. `ListNode` の定義

```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

### 各フィールドの意味

- `val: i32`：ノードが保持する整数値。
- `next: Option<Box<ListNode>>`：次のノードへのリンク。
- `Some(Box::new(...))`：次のノードが存在する。
- `None`：次のノードが存在しない。つまりリストの末尾。

### `#[derive(...)]` の意味

| トレイト | 役割 |
| --- | --- |
| `PartialEq` | `==` や `!=` などの比較を可能にする |
| `Eq` | 同値関係の性質を満たすことを示す |
| `Clone` | `clone()` による複製を可能にする |
| `Debug` | `{:?}` でデバッグ出力できるようにする |

---

## 3\. なぜ `Option<Box<ListNode>>` なのか？

### 3.1 `Option<T>`

`Option<T>` は、値が存在するかどうかを表す型。

```
Some(10) // 値が存在する
None     // 値が存在しない
```

連結リストでは、末尾のノードに次のノードが存在しないため、`None` を使う。

```
[1] → [2] → [3] → None
```

### 3.2 `Box<T>`

`Box<T>` は、ヒープ上に確保された値を所有するスマートポインタ。

連結リストでは、各ノードが次のノードを所有するようにする。

```
[1] → [2] → [3] → None
```

`Box<ListNode>` を使うことで、再帰的なデータ構造を定義できる。

もし次のように書くと、

```
struct ListNode {
    val: i32,
    next: Option<ListNode>,
}
```

`ListNode` の中に `ListNode` が直接含まれるため、型のサイズを有限に決められない。

一方、`Box<ListNode>` は一定サイズのポインタなので、再帰的な構造を定義できる。

**覚えること：**

- `Option`：値が存在するかどうかを表す。
- `Box`：ヒープ上の値を所有する。
- `Option<Box<ListNode>>`：次のノードがある場合は所有し、なければ `None` にする。

---

## 4\. `ListNode::new()` と `Self`

```
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}
```

- `impl ListNode`：`ListNode` に関連関数やメソッドを実装する。
- `Self`：実装対象の型である `ListNode` を表す。
- `ListNode::new(5)`：値が `5` で、次のノードを持たないノードを生成する。
- `#[inline]`：コンパイラにインライン化を促すヒント。必ずインライン化されるわけではない。

---

## 5\. Rustの所有権（Ownership）

Rustでは、値の所有者が明確に決まっている。

例えば、

```
let ptr1 = Some(Box::new(ListNode {
    val: 1,
    next: None,
}));
```

このとき、`ptr1` が `Box<ListNode>` の所有権を持つ。

### 所有権を移動するとどうなる？

```
let ptr2 = ptr1;
```

`ptr1` の値の所有権が `ptr2` に移動する。

この場合、`ptr1` はその値を引き続き使えない。

連結リストのマージでは、ノードをコピーするのではなく、所有権を移動してつなぎ直す。

---

## 6\. `unwrap()`、`take()`、`as_ref()`、`as_mut()`

この4つは、今回のコードを理解するうえで特に重要。

### 6.1 `unwrap()`

`Option<T>` から中身の `T` を取り出す。

```
let x: Option<i32> = Some(10);
let y: i32 = x.unwrap();
```

`None` に対して `unwrap()` を実行すると、実行時にパニックする。

### 6.2 `take()`

値を取り出し、元の場所を `None` にする。

```
let mut x: Option<i32> = Some(10);
let y = x.take();

// y = Some(10)
// x = None
```

`take()` は、値を取り出す対象を変更できる必要がある。

連結リストでは、ノードを次の変数に移動させるときに使う。

### 6.3 `as_ref()`

中身を不変参照で借りる。

```
ptr1.as_ref().unwrap().val
```

`ptr1` の所有権を移動せずに、ノードの値を比較できる。

例えば、

```
if ptr1.as_ref().unwrap().val
    <= ptr2.as_ref().unwrap().val
{
    // ...
}
```

とすれば、比較のために `ptr1` や `ptr2` の所有権を移動させずに済む。

### 6.4 `as_mut()`

中身を可変参照で借りる。

```
tail = tail.next.as_mut().unwrap();
```

ノードを取り出さずに、可変参照を使って操作する。

### 使い分け一覧

| メソッド | 役割 |
| --- | --- |
| `unwrap()` | `Option<T>` から `T` を取り出す |
| `take()` | 値を取り出し、元の場所を `None` にする |
| `as_ref()` | 中身を不変参照で借りる |
| `as_mut()` | 中身を可変参照で借りる |

**重要：** `unwrap()` 自体が必ず所有権を移動するわけではない。`Option<T>` の中身が所有値なのか、参照なのかによって結果が異なる。

---

## 7\. `node` を作る理由

マージ処理では、先頭ノードだけを取り出し、残りのリストは次の比較に使いたい。

例えば、

```
ptr1
 ↓
[2] → [4] → [6] → None
```

この状態で、

```
tail.next = ptr1.take();
```

とすると、リスト全体が `tail.next` に移動してしまう。

そこで、一時変数 `node` を使う。

```
let mut node = ptr1.take().unwrap();
ptr1 = node.next.take();
tail.next = Some(node);
```

### STEP 1：先頭ノードを取り出す

```
let mut node = ptr1.take().unwrap();
```

- `ptr1.take()`：`ptr1` からリストを取り出し、`ptr1` を `None` にする。
- `unwrap()`：`Option` の中身である `Box<ListNode>` を取り出す。

この時点では、`node` がリスト全体を所有している。

```
node
 ↓
[2] → [4] → [6] → None

ptr1 = None
```

### STEP 2：残りのリストを切り離す

```
ptr1 = node.next.take();
```

`node.next` から残りのリストを取り出して `ptr1` に移動する。

```
node              ptr1
 ↓                 ↓
[2] → None        [4] → [6] → None
```

### STEP 3：先頭ノードだけをつなぐ

```
tail.next = Some(node);
```

`node` を `Some(...)` で包み、結合リストの末尾につなぐ。

### STEP 4：末尾を進める

```
tail = tail.next.as_mut().unwrap();
```

新しくつないだノードを `tail` が指すようにする。

**`node`****を作る目的は、マージ先に移すノードと、次の処理に回すリストを分離すること。**

---

## 8\. `dummy` と `tail`

```
let mut dummy = ListNode::new(0);
let mut tail = &mut dummy;
```

### `dummy`

結合後のリストの先頭を管理するためのダミーノード。

最初のノードを特別扱いせず、すべてのノードを同じ方法でつなげられる。

最後に `dummy.next` を返すことで、ダミーノード自体は結果に含めない。

### `tail`

現在の結合リストの末尾を指す可変参照。

```
tail.next = Some(node);
tail = tail.next.as_mut().unwrap();
```

1. `tail.next = Some(node)`：ノードを末尾につなぐ。
2. `tail = tail.next.as_mut().unwrap()`：`tail` を新しい末尾へ進める。

`tail` の型は `&mut ListNode` であり、`Box<ListNode>` ではない。

そのため、`tail` に `Box<ListNode>` を直接代入することはできない。

---

## 9\. `while` の条件：`&&` と `||`

### `&&` を使う場合

```
while ptr1.is_some() && ptr2.is_some() {
    // 両方にノードがある間、比較する
}
```

`&&` は「両方とも条件を満たす」という意味。

片方が `None` になった時点でループを終了できる。

その後、残ったリストを丸ごと接続する。

```
tail.next = if ptr1.is_some() {
    ptr1
} else {
    ptr2
};
```

この方法は、今回のマージ処理ではシンプルで効率的。

### `||` を使う場合

```
while ptr1.is_some() || ptr2.is_some() {
    // どちらかにノードが残っていれば繰り返す
}
```

`||` は「どちらか一方でも条件を満たす」という意味。

この場合、片方が `None` でもループは続く。

そのため、値を比較する前に `None` の場合を処理する必要がある。

```
while ptr1.is_some() || ptr2.is_some() {
    if ptr1.is_none() {
        // ptr2 のノードを処理
    } else if ptr2.is_none() {
        // ptr1 のノードを処理
    } else {
        // 両方にノードがあるので値を比較
    }
}
```

**使い分け：**

- `&&`：両方にノードがある間だけ比較する。
- `||`：片方だけ残っている場合もループ内で処理する。

---

## 10\. `merge_two_lists` の完成コード

```
struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ptr1 = list1;
        let mut ptr2 = list2;

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while ptr1.is_some() && ptr2.is_some() {
            let mut node;

            if ptr1.as_ref().unwrap().val
                <= ptr2.as_ref().unwrap().val
            {
                node = ptr1.take().unwrap();
                ptr1 = node.next.take();
            } else {
                node = ptr2.take().unwrap();
                ptr2 = node.next.take();
            }

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        // 残ったリストをそのまま接続する
        tail.next = if ptr1.is_some() {
            ptr1
        } else {
            ptr2
        };

        dummy.next
    }
}
```

### 処理の流れ

1. `ptr1` と `ptr2` の先頭の値を比較する。
2. 小さい方の先頭ノードを `node` に取り出す。
3. `node.next.take()` で残りのリストを元のポインタに戻す。
4. `node` を `tail.next` に接続する。
5. `tail` を新しい末尾に進める。
6. 片方が空になったら、残りのリストをそのまま接続する。
7. `dummy.next` を返す。

最初の `list1 == None`、`list2 == None` の判定も不要。どちらかが空の場合も、この実装で処理できる。

---

## 11\. `main()` でリストを作ってテストする

```
fn main() {
    // list1: 2 → 4 → 6 → None
    let list1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 6,
                next: None,
            })),
        })),
    }));

    // list2: 1 → 3 → 5 → None
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 5,
                next: None,
            })),
        })),
    }));

    println!("{:?}", Solution::merge_two_lists(list1, list2));
}
```

### 実行結果

```
Some(ListNode {
    val: 1,
    next: Some(ListNode {
        val: 2,
        next: Some(ListNode {
            val: 3,
            next: Some(ListNode {
                val: 4,
                next: Some(ListNode {
                    val: 5,
                    next: Some(ListNode {
                        val: 6,
                        next: None
                    })
                })
            })
        })
    })
})
```

※ `println!("{:?}", ...)` は1行で表示することもある。

---

## 12\. 学習時の重要な Tips

### Tips 1：比較するときは参照を使う

```
ptr1.as_ref().unwrap().val
```

値を比較するだけなら、所有権を移動する必要はない。

### Tips 2：ノードを移動するときは `take()` を使う

```
let mut node = ptr1.take().unwrap();
```

所有権を `node` に移し、元の `ptr1` を `None` にする。

### Tips 3：次のノードを確保してから接続する

```
ptr1 = node.next.take();
tail.next = Some(node);
```

先に残りのリストを切り離しておくことで、先頭ノードだけを安全に接続できる。

### Tips 4：`Some(node)` は所有権を移動する

```
tail.next = Some(node);
```

`node` は `tail.next` に移動するため、その後は `node` を直接使えない。

### Tips 5：`tail` はノードそのものではなく参照

```
let mut tail = &mut dummy;
```

末尾のノードへ進めるときは、

```
tail = tail.next.as_mut().unwrap();
```

とする。

### Tips 6：`unwrap()` の前に `None` の可能性を考える

```
ptr1.take().unwrap()
```

`ptr1` が `None` ならパニックする。

`while ptr1.is_some() && ptr2.is_some()` のように、両方に値が存在する条件を作っておくと、ループ内で安全に処理しやすい。

ただし、コンパイラがあらゆるケースで `unwrap()` の安全性を保証するわけではない。

---

## 13\. 最後に覚えておきたいこと

Rustで連結リストを操作するときは、次の3つを考える。

1. **何を移動したいか？**
   - 今回は先頭ノード1個。
2. **何を残したいか？**
   - 残りのリスト。
3. **次のノードへのつながりを確保したか？**
   - `node.next.take()` で残りのリストを確保する。

基本方針は次のとおり。

- 比較には `as_ref()` を使う。
- ノードの移動には `take()` を使う。
- 末尾の操作には `as_mut()` を使う。
- `Option<T>` の中身を取り出すときは `unwrap()` の前提を確認する。
- `dummy` で先頭ノードの特別扱いをなくし、`tail` で末尾への追加を簡単にする。

**連結リストの問題では、値の比較だけでなく、各ノードの所有権が今どこにあるかを追うことが重要。**


```rust
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
       val,
       next:None
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

        // dummyというリストノードを作り、tailという可変参照でいじる
        // 返すのはdummy.next
        let mut dummy = ListNode::new(0);
        // これならptr1,2みたいにOption<Box<ListNode>>型じゃないからListNodeに直接アクセスできる
        let mut tail = &mut dummy; // tail.next == (*tail).nextになる
        let mut node : Box<ListNode> ;


        // ptr1.unwrap().val > ptr2.unwrap().val は、ptr1、2の所有権を奪ってしまう。(ptr1、2がもう使えなくなってしまう)
        // as_ref()を使うと所有権を奪うことなくunwrap()ができる

        // take() は、値を取り出す対象を変更できる必要がある。しかし、as_ref() の後は不変参照になっているので、そこから take() することはできない。
        // つまり、値を比較するときは as_ref()、所有権を移動するときは take() と使い分けよう。
        /*
        unwrap()：Option<T> から T を取り出す。

        take()：値を取り出し、元の場所を None にする。

        as_ref()：中身を参照して、所有権を移動せずに扱う。
         */
        if ptr1.as_ref().unwrap().val > ptr2.as_ref().unwrap().val{
            // tail = ptr2.as_ref().unwrap().take(); // これはas_ref()の後にunwrap()したものをtake()でNoneに使用としているからダメ
            // tail.next = ptr2.take().unwrap()でここでunwrap()を付けると左辺はOptionなのが一致しなくなる
            node = ptr2.take().unwrap();
            ptr2 = node.next.take(); // nodeの後ろのリストだけをptr2に渡す
            tail.next = Some(node); // tailはListNode,tail.nextはOption<Box<ListNode>>
            tail = tail.next.as_mut().unwrap(); 

            // nodeを作ることで、ptr1,2だとその後ろのリストもすべて持ってきてしまうから、先頭ノードとその後ろのノードを分離したい。
            // node.nextをNoneにすることで、それが叶う。
        }

        else{
            node = ptr1.take().unwrap();
            ptr1 = node.next.take();
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }


        while ptr1 != None || ptr2 != None{

            if ptr2 == None{
                node = ptr1.take().unwrap();
                ptr1 = node.next.take();
                tail.next = Some(node);
                
                // if ptr1 != None // このif文もいらない、なぜならwhile文の条件式より両方がNoneのことはないから
            }

            else if ptr1 == None{
                node = ptr2.take().unwrap();
                // ptr2 = node.next.take();ここでtake()を付けないと、構造体のフィールドの所有権が移ると構造体自体の所有権が移る。
                // そうすると下のSome(node)でエラーが出る
                ptr2 = node.next.take(); 
                tail.next = Some(node);
            }

            else if ptr1.as_ref().unwrap().val > ptr2.as_ref().unwrap().val{
                node = ptr2.take().unwrap();
                ptr2 = node.next.take();
                tail.next = Some(node);
            }

            else if ptr2.as_ref().unwrap().val >= ptr1.as_ref().unwrap().val{
                node = ptr1.take().unwrap();
                ptr1 = node.next.take();
                tail.next = Some(node);
            }

            // tailは&mut ListNodeだからas_mut()でmutにする。
            tail = tail.next.as_mut().unwrap();
        }

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

             return dummy.next
        }



    }



fn main(){
    // 前提として二つのリストはsortされている。
    let list1 = Some(Box::new(ListNode{val:2,
        next:Some(Box::new(ListNode{val:4,
            next:Some(Box::new(ListNode{val:6,
                next:None}))}))}));

    let list2 = Some(Box::new(ListNode{val:1,
        next:Some(Box::new(ListNode{val:3,
            next:Some(Box::new(ListNode{val:5,
                next:None}))}))}));

    println!("{:?}",Solution::merge_two_lists(list1,list2));
}
```