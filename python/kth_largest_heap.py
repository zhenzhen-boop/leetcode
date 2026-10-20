class KthLargest:
    
    ## heapqを使ってみる
    def __init__(self, k: int, nums: list[int]):
        self.k = k
        self.heap = nums
        heapq.heapify(self.heap) # これでリストをヒープに変換
        
        # 最小の値が先頭に（1番上に）あるから残りの要素数がk個になるまで1番上の要素をさよならする
        # -> そうするとk番目の値が1番上にくる
        
        while len(self.heap) > k:
            heapq.heappop(self.heap)

    def add(self, val: int) -> int:
        heapq.heappush(self.heap,val)
        if len(self.heap) > self.k:
            heapq.heappop(self.heap)
        
        return self.heap[0]

'''
• 追加: heapq.heappush(h, x) — \(O(\log N)\) で値 x を追加。
• 最小値の取り出し: heapq.heappop(h) — \(O(\log N)\) で最小値を取り出して削除。
• 最小値の参照: h[0] — O(1) で削除せずに確認。
• ヒープ化: heapq.heapify(h) — O(N) でリストを変換。
• 追加して取り出す: heapq.heappushpop(h, x) — \(O(\log N)\)。
'''
# Your KthLargest object will be instantiated and called as such:
# obj = KthLargest(k, nums)
# param_1 = obj.add(val)