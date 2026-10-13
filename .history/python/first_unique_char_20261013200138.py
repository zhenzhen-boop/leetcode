class Solution(object):
    def firstUniqChar(self, s):
        count = Counter(s)
        # Counter({'a': 2, 'b': 2, 'c': 2, 'd': 1})となる
        
        # find the index
        for idx, ch in enumerate(s):
            if count[ch] == 1:
                return idx     
        return -1
  


        """
        :type s: str
        :rtype: int
        """
        