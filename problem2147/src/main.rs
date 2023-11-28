impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: usize = 1_000_000_007;

        let mut seat_count = 0;
        let mut total_seat_count = 0;
        let mut plant_count = 0;
        let mut ret = 1;
        for c in corridor.chars() {
            match c {
                'S' => {
                    if seat_count == 2 {
                        ret = (ret * (plant_count + 1)) % MOD;
                        plant_count = 0;
                        seat_count = 1;
                    } else {
                        seat_count += 1;
                    }
                    total_seat_count += 1;
                },
                'P' => {
                    if seat_count == 2 {
                        plant_count += 1;
                    }
                },
                _ => panic!("Only P and S are allowed"),
            }
        }
        if total_seat_count >= 2 && total_seat_count % 2 == 0 {
            return ret as i32;
        } else {
            return 0;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::number_of_ways("S".to_string()), 0);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::number_of_ways("P".to_string()), 0);
    }
}

struct Solution {}
fn main() {
    println!("Hello, world!");
}
