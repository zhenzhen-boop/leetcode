class Solution:
# 返り値は何？型は？
    def removeDuplicates(self, nums: list[int]) -> int:
        set_nums = set(nums)
        arr = []

        for num in set_nums:
            arr.append(num)

        arr = sorted(arr)

        for _ in range(len(arr)+1,len(nums)):
            arr.append("_")

        return arr


def main():
    print(Solution().removeDuplicates([1,1,2,3,4,5,5]))

if __name__ == "__main__":
    main()        