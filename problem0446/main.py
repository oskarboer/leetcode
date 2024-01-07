class Solution:
    def numberOfArithmeticSlices(self, nums: List[int]) -> int:

        dp = [dict() for i in nums]
        total = 0

        for i in range(1, len(nums)):
            for j in range(i):
                diff = nums[i] - nums[j]

                dp[i].setdefault(diff, 0)
                dp[i][diff] += dp[j].get(diff, 0) + 1
                total += dp[j].get(diff, 0)

        return total
        
