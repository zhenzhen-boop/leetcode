Python heapq と KthLargest の実装まとめ
1. heapq とは？

Python 標準ライブラリの heapq は、ヒープ（Heap）というデータ構造を扱うためのモジュール。

ヒープを使うと、最小値を効率よく取り出せる。

import heapq
最小ヒープ（Min Heap）

Python の heapq は、デフォルトで 最小ヒープ を使用する。

最小値が heap[0] にある。
リスト全体が昇順にソートされているわけではない。
要素の追加・削除を効率よく行える。

例えば、次のリストをヒープ化する。

import heapq

heap = [4, 5, 8, 2]
heapq.heapify(heap)

print(heap[0])  # 2


heapify() はリストをヒープ構造に変換する。

ヒープ化後のリストの並び順は、常に同じソート順になるとは限らない。ただし、最小値は必ず先頭にある。

2. 覚えておきたい heapq の関数
関数	役割	時間計算量
heapq.heapify(h)	リストをヒープ化する	O
(
N
)

heapq.heappush(h, x)	要素 x を追加する	O
(
log
⁡
N
)

heapq.heappop(h)	最小値を取り出して削除する	O
(
log
⁡
N
)

h[0]	最小値を参照する	O
(
1
)

heapq.heappushpop(h, x)	要素を追加し、最小値を取り出す	O
(
log
⁡
N
)

※ N は操作時点のヒープの要素数。

① heapify()：リストをヒープに変換
heap = [4, 5, 8, 2]
heapq.heapify(heap)


リストを新しく作らず、既存のリストをその場でヒープ化する。

計算量は O
(
N
)
。

② heappush()：要素を追加
heapq.heappush(heap, 3)


要素を追加し、ヒープの条件を保つ。

計算量は O
(
log
⁡
N
)
。

③ heappop()：最小値を取り出す
smallest = heapq.heappop(heap)


最小値を返すと同時に、ヒープから削除する。

計算量は O
(
log
⁡
N
)
。

④ heap[0]：最小値を参照する
smallest = heap[0]


最小値を確認するだけで、削除はしない。

計算量は O
(
1
)
。

⑤ heappushpop()：追加と取り出しをまとめる
smallest = heapq.heappushpop(heap, 10)


要素 10 を追加した場合と同等の処理を行い、最小値を返してヒープから取り除く。

この操作では、ヒープの要素数は基本的に変わらない。

3. KthLargest 問題とは？

ストリーム状に数値が追加されるとき、大きい方から k 番目の値を求める問題。

例えば、k = 3 の場合：

数値の集合	大きい順	答え
[4, 5, 8, 2]	8, 5, 4, 2	4
[4, 5, 8, 2, 10]	10, 8, 5, 4, 2	5

毎回すべての数値をソートすると効率が悪い。

そこで、大きい方から k 個だけを最小ヒープで保持する。

ヒープの最小値が、全体の k 番目に大きい値になる。

4. コード全体
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
5. __init__() の重要ポイント
self.k = k
self.heap = nums
heapq.heapify(self.heap)

while len(self.heap) > k:
    heapq.heappop(self.heap)
処理の流れ
self.k = k：保持する要素数の上限を記録する。
self.heap = nums：初期値を保存する。
heapq.heapify(self.heap)：最小ヒープに変換する。
while len(self.heap) > k：要素数が k を超えている間、最小値を削除する。

例えば、k = 3、nums = [4, 5, 8, 2] の場合：

初期値は [4, 5, 8, 2]
最小値 2 を削除
上位 3 個の値 4, 5, 8 が残る

ヒープの中身はソート済みではないが、最小値の 4 が先頭にある。

したがって、heap[0] が答えになる。

6. add() の重要ポイント
def add(self, val: int) -> int:
    heapq.heappush(self.heap, val)

    if len(self.heap) > self.k:
        heapq.heappop(self.heap)

    return self.heap[0]
処理の流れ
新しい値をヒープに追加する。
要素数が k を超えた場合、最小値を削除する。
heap[0] を返す。

例えば、k = 3、現在のヒープが [4, 5, 8] のとき、3 が追加された場合：

追加前：       [4, 5, 8]
3 を追加：     [3, 4, 8, 5]  # ヒープの並びの一例
最小値 3 を削除
残る値：       [4, 5, 8]
返り値：       4


3 は上位 3 個に入らないため削除される。

このように、ヒープのサイズを k 以下に保つことで、常に答えを効率よく取得できる。

heappushpop() を使った書き方
def add(self, val: int) -> int:
    if len(self.heap) < self.k:
        heapq.heappush(self.heap, val)
    else:
        heapq.heappushpop(self.heap, val)

    return self.heap[0]


ヒープがすでに k 個ある場合、heappushpop() で追加と最小値の取り出しをまとめられる。

ヒープが k 個未満の場合は、削除せずに追加する。

7. 計算量

初期値の個数を N、保持する要素数を k とする。

処理	時間計算量
heapify()	O
(
N
)

初期化時の要素削除	O
(
(
N
−
k
)
log
⁡
N
)
（N > k の場合の上界）
add()	O
(
log
⁡
k
)

heap[0]	O
(
1
)

ヒープの空間計算量	O
(
k
)

add() ではヒープの要素数が最大 k 個程度なので、追加と削除は O
(
log
⁡
k
)
 で行える。

初期化時の削除回数は、正確には max(0, N-k) 回。

8. よくある注意点
① ヒープはソート済みリストではない
heapq.heapify(nums)


を実行しても、リスト全体が昇順になるわけではない。

最小値が heap[0] にあることが重要。

② heappop() は値を返しながら削除する
x = heapq.heappop(heap)


x に最小値が入り、その値はヒープから削除される。

③ heap[0] は削除しない
x = heap[0]


単に最小値を参照するだけ。

④ heap が空だと heap[0] はエラーになる

初期値が空で add() も呼ばれていない場合などに注意する。

通常の KthLargest 問題では k >= 1 が前提となる。

⑤ self.heap = nums は同じリストを参照する

self.heap と nums は同じリストを指す。そのため、heapify() や heappop() によって元の nums も変更される。

元のリストを変更したくない場合は、次のようにコピーする。

self.heap = nums.copy()
9. この問題で最も大切な考え方

最小ヒープのサイズを k 個に制限する。

最小ヒープなので、保持している値の中で最小のものが先頭にある。
上位 k 個だけを保持する。
その中の最小値が、全体の k 番目に大きい値になる。

つまり、毎回すべてをソートするのではなく、必要な k 個だけを効率よく管理するのがこの問題の核心。