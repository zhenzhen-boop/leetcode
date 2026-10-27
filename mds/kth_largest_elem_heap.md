# Python `heapq` と KthLargest のまとめ

## 1\. `heapq` とは？

Python 標準ライブラリの `heapq` は、ヒープ（Heap）というデータ構造を扱うためのモジュール。

Python の `heapq` は **最小ヒープ（Min Heap）** を使用する。

- 最小値が `heap[0]` にある
- リスト全体がソートされているわけではない
- 最小値の追加・削除を効率よく行える

```
import heapq

heap = [4, 5, 8, 2]
heapq.heapify(heap)

print(heap[0])  # 2
```

## 2\. `heapq` の基本操作

| 関数 | 役割 | 時間計算量 |
| --- | --- | --- |
| `heapq.heapify(h)` | リストをヒープ化する | `O(N)` |
| `heapq.heappush(h, x)` | 要素 `x` を追加する | `O(log N)` |
| `heapq.heappop(h)` | 最小値を取り出して削除する | `O(log N)` |
| `h[0]` | 最小値を参照する | `O(1)` |
| `heapq.heappushpop(h, x)` | 追加と最小値の取り出しをまとめる | `O(log N)` |

### `heapify()`：リストをヒープ化する

```
heapq.heapify(heap)
```

既存のリストをその場でヒープ構造に変換する。

### `heappush()`：要素を追加する

```
heapq.heappush(heap, 3)
```

要素を追加し、ヒープの条件を維持する。

### `heappop()`：最小値を取り出す

```
smallest = heapq.heappop(heap)
```

最小値を返すと同時に、ヒープから削除する。

### `heap[0]`：最小値を参照する

```
smallest = heap[0]
```

最小値を確認するだけで、削除はしない。

### `heappushpop()`：追加と取り出しをまとめる

```
smallest = heapq.heappushpop(heap, 10)
```

値を追加し、ヒープから最小値を取り出して返す。ヒープの要素数は変わらない。

---

## 3\. KthLargest 問題とは？

数値が追加されていくとき、**大きい方から****`k`****番目の値**を求める問題。

例えば、`k = 3` の場合：

| 数値の集合 | 大きい順 | 答え |
| --- | --- | --- |
| `[4, 5, 8, 2]` | `8, 5, 4, 2` | `4` |
| `[4, 5, 8, 2, 10]` | `10, 8, 5, 4, 2` | `5` |

毎回すべての数値をソートする代わりに、**大きい方から****`k`****個だけを最小ヒープで保持する**。

上位 `k` 個の中で最小の値が、全体の `k` 番目に大きい値になる。

---

## 4\. コード全体

```
import heapq

class KthLargest:

    def __init__(self, k: int, nums: list[int]):
        self.k = k
        self.heap = nums

        # リストを最小ヒープに変換
        heapq.heapify(self.heap)

        # 要素数が k を超えたら最小値を削除
        while len(self.heap) > k:
            heapq.heappop(self.heap)

    def add(self, val: int) -> int:
        # 新しい値を追加
        heapq.heappush(self.heap, val)

        # 要素数が k を超えたら最小値を削除
        if len(self.heap) > self.k:
            heapq.heappop(self.heap)

        # k 番目に大きい値を返す
        return self.heap[0]
```

---

## 5\. `__init__()` の重要ポイント

```
self.k = k
self.heap = nums
heapq.heapify(self.heap)

while len(self.heap) > k:
    heapq.heappop(self.heap)
```

### 処理の流れ

1. `self.k = k`：保持する要素数の上限を記録する。
2. `self.heap = nums`：初期値を保存する。
3. `heapq.heapify(self.heap)`：最小ヒープに変換する。
4. `while len(self.heap) > k`：要素数が `k` を超えている間、最小値を削除する。

例：`k = 3`、`nums = [4, 5, 8, 2]`

- 初期値は `[4, 5, 8, 2]`
- 最小値 `2` を削除する
- 上位 3 個の値 `4, 5, 8` が残る

ヒープの中身はソート済みではないが、最小値の `4` が先頭にある。

そのため、`heap[0]` が答えになる。

---

## 6\. `add()` の重要ポイント

```
def add(self, val: int) -> int:
    heapq.heappush(self.heap, val)

    if len(self.heap) > self.k:
        heapq.heappop(self.heap)

    return self.heap[0]
```

### 処理の流れ

1. 新しい値をヒープに追加する。
2. 要素数が `k` を超えたら最小値を削除する。
3. `heap[0]` を返す。

例：`k = 3`、現在のヒープが `[4, 5, 8]` の場合

`val = 3` を追加すると：

```
追加前：       [4, 5, 8]
3 を追加：     [3, 4, 8, 5]  # ヒープの並びの一例
最小値 3 を削除
残る値：       [4, 5, 8]
返り値：       4
```

`3` は上位 3 個に入らないため削除される。

このように、ヒープのサイズを `k` 以下に保つことで、答えを効率よく取得できる。

### `heappushpop()` を使った書き方

```
def add(self, val: int) -> int:
    if len(self.heap) < self.k:
        heapq.heappush(self.heap, val)
    else:
        heapq.heappushpop(self.heap, val)

    return self.heap[0]
```

- ヒープが `k` 個未満なら、値を追加する。
- すでに `k` 個あるなら、`heappushpop()` で追加と最小値の取り出しをまとめる。

---

## 7\. 計算量

初期値の個数を `N`、保持する要素数を `k` とする。

| 処理 | 時間計算量 |
| --- | --- |
| `heapify()` | `O(N)` |
| 初期化時の要素削除 | `O((N-k) log N)`（`N > k` の場合の上界） |
| `add()` | `O(log k)` |
| `heap[0]` | `O(1)` |
| ヒープの空間計算量 | `O(k)` |

`add()` ではヒープの要素数が最大 `k` 個程度なので、追加・削除は `O(log k)` で行える。

初期化時の削除回数は `max(0, N-k)` 回。

---

## 8\. 注意点

### ① ヒープはソート済みリストではない

`heapq.heapify(nums)` を実行しても、リスト全体が昇順になるわけではない。

重要なのは、最小値が `heap[0]` にあること。

### ② `heappop()` は削除する

```
x = heapq.heappop(heap)
```

最小値を `x` に格納し、その値をヒープから削除する。

### ③ `heap[0]` は削除しない

```
x = heap[0]
```

最小値を参照するだけ。

### ④ ヒープが空だと `heap[0]` はエラーになる

初期値が空で `add()` も呼ばれていない場合などに注意する。

通常の KthLargest 問題では `k >= 1` が前提となる。

### ⑤ `self.heap = nums` は同じリストを参照する

`self.heap` と `nums` は同じリストを指すため、`heapify()` や `heappop()` で元のリストも変更される。

元のリストを変更したくない場合は、コピーする。

```
self.heap = nums.copy()
```

---

## 9\. この問題で最も大切な考え方

**最小ヒープのサイズを****`k`****個に制限する。**

- 最小ヒープでは、最小値が先頭にある。
- 大きい方から `k` 個だけを保持する。
- その中の最小値が、全体の `k` 番目に大きい値になる。

つまり、毎回すべてをソートするのではなく、**必要な****`k`****個だけを効率よく管理する**ことが、この問題の核心。