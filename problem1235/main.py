from bisect import bisect_right, bisect_left

class Job:
    def __init__(self, s, e, p, prev=None):
        self.start = s
        self.end = e
        self.profit = p
        self.prev = prev

    def __str__(self):
        return f"start-end: ({self.start} {self.end}), profit-value ({self.profit}, {self.value}); prev: {self.prev}"

class Solution:
    def jobScheduling(self, startTime: List[int], endTime: List[int], profit: List[int]) -> int:
        n = len(startTime)
        

        jobs = [Job(s, e, p) for s, e, p in zip(startTime, endTime, profit)]
        jobs.sort(key=lambda j: j.end)

        dp = [[jobs[i].profit, 0] for i in range(n)]

        for i, j in enumerate(jobs):
            point = bisect_right(jobs, j.start, key=lambda e: e.end) - 1
            if point >= 0:
                j.prev = point if jobs[point].end <= j.start and i != point else None

        for i, j in enumerate(jobs[1:]):
            if j.prev != None:
                dp[i+1][0] = max(dp[i+1][0], max((j.profit) + max(dp[j.prev]),  max(dp[i])))
            dp[i+1][1] = max(dp[i])

        return max(max(dp, key=lambda e: max(e)))