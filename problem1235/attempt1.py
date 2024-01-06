from bisect import bisect_right

class Job:
    def __init__(self, s, e, p, prev=None):
        self.start = s
        self.end = e
        self.profit = p
        self.prev = prev
        self.value = p
    def __str__(self):
        return f"start-end: ({self.start} {self.end}), profit-value ({self.profit}, {self.value}); prev: {self.prev}"

class Solution:
    def jobScheduling(self, startTime: List[int], endTime: List[int], profit: List[int]) -> int:

        jobs = [Job(s, e, p) for s, e, p in zip(startTime, endTime, profit)]

        jobs.sort(key=lambda j: j.end)

        for j in jobs:
            j.prev = bisect_right(jobs, j.start, key=lambda e: e.end)

        for j in jobs:
            j.value += max(jobs[0:j.prev], key=lambda e: e.value, default=Job(0, 0, 0, 0)).value

        return max(jobs, key=lambda j: j.value).value