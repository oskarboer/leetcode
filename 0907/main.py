class Solution:
    def sumSubarrayMins(self, arr: List[int]) -> int:
        MOD = 1_000_000_007
        result = 0

        # Monothonic stack (instead of storing just values we also have to store counts for them)
        ms = []
        ms_sum = 0 # sum of stack stored separetly to decrease computation strain

        for e in arr:
            new_val = e
            new_count = 1
            while len(ms) > 0 and e < ms[-1][0]:  # Monothonic stack upkeep
                val, count = ms.pop(-1)
                ms_sum -= val * count
                new_count += count
            ms.append((new_val, new_count))
            ms_sum += new_val * new_count

            result = (ms_sum + result) % MOD
        return result
