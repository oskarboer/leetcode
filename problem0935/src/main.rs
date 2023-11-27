impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        
        let mut count: [usize; 10] = [1; 10];


        for _ in 1..n {
            let mut tmp = [0; 10];

            tmp[0] += (count[6] + count[4]) % modulo;
            tmp[1] += (count[6] + count[8]) % modulo;
            tmp[2] += (count[7] + count[9]) % modulo;
            tmp[3] += (count[4] + count[8]) % modulo;
            tmp[4] += (count[3] + count[9] + count[0]) % modulo;
            tmp[5] = 0;
            tmp[6] += (count[1] + count[7] + count[0]) % modulo;
            tmp[7] += (count[2] + count[6]) % modulo;
            tmp[8] += (count[1] + count[3]) % modulo;
            tmp[9] += (count[2] + count[4]) % modulo;

            count = tmp;
        }

        count.iter().fold(0, |acc, x| (acc + *x) % modulo) as i32
    }
}



struct Solution {}

fn main() {
    println!("1: {}", Solution::knight_dialer(1));
    println!("2: {}", Solution::knight_dialer(2));

    println!("3131: {}", Solution::knight_dialer(3131));
}
