impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: usize = 1_000_000_007;
        let (n, k, t) = (n as usize, k as usize, target as usize);

        (0..t).fold(0, |a, i| {
            0
        })
    }
}

struct Solution {}
fn main() {
    println!("Hello, world!");
}
