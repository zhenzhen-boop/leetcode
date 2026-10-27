# Rust学習メモ：Zigzag Conversion（ジグザグ変換）

## 1. 問題の概要

文字列 `s` を指定された行数 `num_rows` でジグザグ状に並べ、各行を上から順番に連結する問題。

例：`s = "PAYPALISHIRING"`、`num_rows = 3`

```text
P   A   H   N
A P L S I I G
Y   I   R
```

各行を上から連結すると、次の文字列になる。

```text
PAHNAPLSIIGYIR
```

---

## 2. コード全体の構造

```rust
struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // ジグザグ変換の処理
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::convert("kawaiidakeja,damedesuka".to_string(), 3)
    );
}
```

### `struct Solution {}`

- `Solution` という構造体を定義する。
- LeetCodeでは、指定された構造体に解答用メソッドを実装する形式がよく使われる。
- 今回は構造体自体にデータを持たせないため、空の構造体になっている。

### `impl Solution`

`Solution` にメソッドを実装するためのブロック。

### `pub fn convert(s: String, num_rows: i32) -> String`

- `pub`：メソッドを公開する。
- `fn`：関数を定義する。
- `s: String`：変換対象の文字列。
- `num_rows: i32`：行数を表す符号付き32ビット整数。
- `-> String`：戻り値の型は `String`。

`String` は所有権を持つ文字列型で、変更可能な文字列を扱える。

---

## 3. 最初の条件分岐：早期リターン

```rust
if num_rows == 1 || num_rows >= s.len() as i32 {
    return s;
}
```

ジグザグ変換をする必要がない場合は、そのまま元の文字列を返す。

### `num_rows == 1`

1行しかない場合、すべての文字が同じ行に並ぶため、文字列は変化しない。

### `num_rows >= s.len() as i32`

行数が文字列の長さ以上なら、ジグザグの折り返しが発生しないため、そのまま返せる。

### `as i32` とは？

```rust
s.len() as i32
```

`as` を使って型を変換している。

- `s.len()` の戻り値は `usize`。
- `num_rows` は `i32`。
- 比較するために型をそろえている。

**注意点**

- `String::len()` は文字数ではなくバイト数を返す。
- 日本語などのマルチバイト文字では、バイト数と文字数が異なる。
- 大きな値を `usize` から `i32` に変換すると、値の範囲に関する問題が起こる可能性がある。

---

## 4. `Vec<String>` で各行を管理する

```rust
let num_rows_usize: usize = num_rows as usize;

let mut arr: Vec<String> = vec!["".to_string(); num_rows_usize];
```

### `usize` とは？

`usize` は、配列やベクターのインデックス、要素数などによく使われる符号なし整数型。

```rust
let idx: usize = 0;
```

負の値は表現できないため、インデックスの計算で `-1` を使う場合は注意が必要。

### `Vec<String>` とは？

`String` を要素として持つ可変長配列。

例えば `num_rows = 3` なら、初期状態は次のようになる。

```text
arr[0] = ""
arr[1] = ""
arr[2] = ""
```

それぞれの要素をジグザグの各行に対応させる。

- `arr[0]`：1行目
- `arr[1]`：2行目
- `arr[2]`：3行目

### `vec!["".to_string(); num_rows_usize]` の意味

`vec![値; 個数]` は、指定した値を指定した個数分並べた `Vec` を作るマクロ。

```rust
vec![0; 3]
// [0, 0, 0]

vec!["".to_string(); 3]
// ["", "", ""]
```

`String` は `Copy` 型ではないが、`Clone` を実装しているため、この形式で複数の空文字列を作れる。

### なぜ `mut` が必要？

```rust
let mut arr: Vec<String>
```

後から `arr` の要素を書き換えるため。

`mut` がないと、`arr[idx]` に新しい値を代入できない。

---

## 5. `step` と `idx` の役割

```rust
let mut step: usize = 2;
let mut idx: usize = 0;
```

- `idx`：現在、文字を追加する行のインデックス。
- `step`：次の行へ移動するときの計算に使う値。

今回のコードでは、次の式で行を移動する。

```rust
idx = idx + step - 1;
```

`step` を切り替えることで、下方向と上方向への移動を表現している。

| `step` | 移動量 | 例 |
|---|---:|---|
| `2` | `+1` | `0 → 1 → 2` |
| `0` | `-1` | `2 → 1 → 0` |

`usize` には負の値がないため、`idx - 1` を直接使わず、`idx + step - 1` としている。

この式は左から順番に計算される。

例えば `idx = 2`、`step = 0` の場合：

```text
2 + 0 - 1 = 1
```

となり、1つ上の行に移動できる。

ただし、`idx = 0`、`step = 0` で計算するとアンダーフローが起きる。

今回のコードでは、計算前に `idx == 0` の場合の `step` を `2` に変更することで、この状態を避けている。

---

## 6. 文字列を1文字ずつ処理する

```rust
for (_, ch) in s.chars().enumerate() {
    arr[idx] = arr[idx].clone() + &(ch.to_string());
}
```

### `s.chars()`

文字列から文字を1つずつ取り出すイテレータを作る。

例えば `"ABC"` なら、順番に `A`、`B`、`C` を取り出す。

### `.enumerate()`

各要素にインデックスを付ける。

```rust
for (i, ch) in s.chars().enumerate() {
    // i は文字の位置
    // ch は現在の文字
}
```

今回のコードではインデックスを使わないため、`_` で受け取っている。

```rust
for (_, ch) in s.chars().enumerate()
```

実際には、次のように簡潔に書ける。

```rust
for ch in s.chars()
```

### `ch.to_string()`

`ch` は `char` 型の1文字。`to_string()` で `String` に変換している。

### `clone()` が必要な理由

```rust
arr[idx] = arr[idx].clone() + &(ch.to_string());
```

現在の行の文字列を複製し、新しい文字を連結してから、その結果を `arr[idx]` に代入している。

`String` は所有権を持つ型なので、所有権や借用を意識する必要がある。`clone()` によって元の文字列のコピーを作り、そのコピーに文字を連結している。

ただし、今回の処理では `clone()` は必要ない。

`push()` を使えば、現在の行に直接1文字追加できる。

```rust
arr[idx].push(ch);
```

こちらのほうが簡潔で、毎回文字列を複製する必要もない。

---

## 7. 行の方向を切り替える

```rust
if idx == num_rows_usize - 1 {
    step = 0;
} else if idx == 0 {
    step = 2;
}

idx = idx + step - 1;
```

ここがジグザグの動きを作っている部分。

### `idx == num_rows_usize - 1`

最下行に到達した場合。

例えば3行なら、インデックスは `0`、`1`、`2` なので、最下行は `num_rows_usize - 1`、つまり `2`。

```rust
step = 0;
```

これによって、次の行への移動量を `-1` に切り替える。

### `idx == 0`

最上行に到達した場合。

```rust
step = 2;
```

これによって、次の行への移動量を `+1` に切り替える。

### 3行の場合の動き

| 現在の `idx` | `step` | 次の `idx` | 動き |
|---:|---:|---:|---|
| 0 | 2 | 1 | 下へ |
| 1 | 2 | 2 | 下へ |
| 2 | 0 | 1 | 上へ |
| 1 | 0 | 0 | 上へ |
| 0 | 2 | 1 | 下へ |

この繰り返しで、次のように移動する。

```text
0 → 1 → 2 → 1 → 0 → 1 → 2 → ...
```

行番号を更新する前に `step` を切り替えるため、最上行と最下行で正しい方向に折り返せる。

---

## 8. 最後に各行を連結する

```rust
let ans = arr.join("").to_string();

return ans;
```

### `join("")`

`Vec<String>` の要素を、指定した文字列を区切り文字として連結する。

```rust
let arr = vec![
    "PAHN".to_string(),
    "APLSIIG".to_string(),
    "YIR".to_string(),
];

let ans = arr.join("");
```

結果：

```text
PAHNAPLSIIGYIR
```

区切り文字を指定することもできる。

```rust
arr.join(",")
```

結果：

```text
PAHN,APLSIIG,YIR
```

### `.to_string()` は必要？

今回の `arr.join("")` は、すでに `String` を返す。

そのため、後ろの `.to_string()` は不要。

```rust
let ans = arr.join("");
```

さらに、最後の式を直接戻り値にすれば、`return` も省略できる。

```rust
arr.join("")
```

Rust では、関数の最後にセミコロンを付けずに式を書くと、その値が戻り値になる。

---

## 9. `main()` の役割

```rust
fn main() {
    println!(
        "{:?}",
        Solution::convert(
            "kawaiidakeja,damedesuka".to_string(),
            3
        )
    );
}
```

`main()` はプログラムの実行開始地点。

- `"kawaiidakeja,damedesuka".to_string()`：文字列リテラルを `String` に変換する。
- `3`：ジグザグ変換の行数を指定する。
- `Solution::convert(...)`：変換処理を呼び出す。
- `println!`：結果を標準出力に表示する。

### `{:?}` と `{}` の違い

- `{}`：`Display` による通常の表示。
- `{:?}`：`Debug` によるデバッグ用の表示。

今回の戻り値は `String` なので、どちらでも文字列を表示できる。

通常の出力なら、次の書き方が自然。

```rust
println!("{}", Solution::convert(...));
```

---

## 10. 改善版コード

元のコードは、`step` を `0` と `2` に切り替えることで方向を表現している。

ただし、移動式が少し複雑なので、方向を `bool` で管理すると読みやすくなる。

```rust
struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let char_count = s.chars().count();

        if num_rows <= 1 || num_rows as usize >= char_count {
            return s;
        }

        let n = num_rows as usize;
        let mut arr: Vec<String> = vec![String::new(); n];

        let mut idx: usize = 0;
        let mut going_down = true;

        for ch in s.chars() {
            arr[idx].push(ch);

            if idx == 0 {
                going_down = true;
            } else if idx == n - 1 {
                going_down = false;
            }

            if going_down {
                idx += 1;
            } else {
                idx -= 1;
            }
        }

        arr.join("")
    }
}

fn main() {
    println!(
        "{}",
        Solution::convert("kawaiidakeja,damedesuka".to_string(), 3)
    );
}
```

### 改善したポイント

1. `arr[idx].clone() + ...` を `arr[idx].push(ch)` に変更。
2. `enumerate()` を省略し、使わないインデックスをなくした。
3. `step` の計算をやめ、`going_down` で移動方向を管理。
4. `arr.join("").to_string()` を `arr.join("")` に変更。
5. `s.len()` の代わりに `s.chars().count()` を使い、文字数で比較。
6. `num_rows <= 1` とすることで、行数が1以下の場合も安全に元の文字列を返す。

この改善版は、行数が2以上で、文字数が行数を超える場合にジグザグ変換を行う。

---

## 11. 計算量

文字数を `n`、行数を `r` とする。

- **時間計算量：O(n)**
  - 各文字を1回ずつ行に追加し、最後に各行を連結する。
- **空間計算量：O(n + r)**
  - 各行に文字を保存し、行を管理するための領域を使う。

通常、文字列の保存領域が支配的になる。

---

## 12. 覚えておきたいポイントまとめ

| 項目 | 覚えること |
|---|---|
| `Vec<String>` | 文字列を複数行に分けて管理できる |
| `vec![値; 個数]` | 同じ初期値を指定した個数分作る |
| `usize` | インデックスや要素数によく使う符号なし整数型 |
| `as` | 型変換に使う |
| `s.chars()` | 文字列から文字を1つずつ取り出す |
| `.enumerate()` | 要素にインデックスを付ける |
| `String::push(ch)` | 文字列の末尾に1文字追加する |
| `.clone()` | 値を複製する。今回の追加処理では不要 |
| `.join("")` | 文字列の配列を連結する |
| セミコロンなしの最後の式 | その式の値が関数の戻り値になる |
| `println!("{}", value)` | 通常の表示 |
| `println!("{:?}", value)` | デバッグ用の表示 |

## 最重要ポイント

1. **`Vec<String>` で各行の文字列を管理する。**
2. **`usize` は負の値を表せないため、インデックスの計算に注意する。**
3. **`push()` を使えば、文字列を毎回 `clone()` せずに1文字追加できる。**
4. **`join("")` で各行を連結できる。**
5. **`chars()` と `len()` は異なる。前者は文字単位、後者はバイト単位で扱う。**
6. **方向を `bool` で管理すると、ジグザグの移動処理を直感的に書ける。**