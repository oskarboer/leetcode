import numpy as np

class Solution:
    def findPaths(m: int, n: int, max_move: int, start_row: int, start_column: int) -> int:
        MOD = 1_000_000_007
        score = 0

        curr = 0
        a = np.zeros((m, n, 2), dtype=np.uint)
        a[start_row, start_column, curr] = 1

        mod_add = lambda a, b: ((a % MOD) + (b % MOD)) % MOD

        for turn in range(max_move):
            other = (curr + 1) % 2

            for row in range(0, m):
                curr_row = a[row, :, curr]
                if row - 1 >= 0:
                    a[row - 1, :, other] = mod_add(a[row - 1, :, other], curr_row)
                if row + 1 < m:
                    a[row + 1, :, other] = mod_add(a[row + 1, :, other], curr_row)
            
            for col in range(0, n):
                curr_col = a[:, col, curr]
                if col - 1 >= 0:
                    a[:, col - 1, other] = mod_add(a[:, col - 1, other], curr_col)
                if col + 1 < n:
                    a[:, col + 1, other] = mod_add(a[:, col + 1, other], curr_col)
            
            score = mod_add(score, a[0, :, curr].sum())
            score = mod_add(score, a[-1, :, curr].sum())

            score = mod_add(score, a[:, 0, curr].sum())
            score = mod_add(score, a[:, -1, curr].sum())

            a[:, :, curr] = 0
            curr = other
        
        return int(score)

if __name__ == "__main__":
    Solution.findPaths(50, 50, 50, 0, 20)