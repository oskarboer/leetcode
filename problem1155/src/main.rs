impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: usize = 1_000_000_007;
        let (n, k, t) = (n as usize, k as usize, target as usize);

        let mut acc = vec![vec![0usize; t]; 2];
        for i in 0..k.min(t) {
            acc[1][i] = 1;
        }
        println!("{:?}", acc[1]);

        for dice in 2..=n {
            for s in (dice - 1)..(dice*k).min(t) {
                let start = if s > k {s - k} else {0};
                for i in 0..(dice - 1) {
                    acc[dice % 2][i] = 0
                }
                acc[dice % 2][s] = acc[(dice - 1) % 2][start..s].iter().fold(0, |a, x| {(a + x) % MOD});
            }
            println!("{:?}", acc[dice % 2]);
        }
        
        return acc[n % 2][t - 1] as i32;
    }
}

struct Solution {}
fn main() {
    println!("{}", Solution::num_rolls_to_target(6, 6, 17));
}
