pub struct Solution;
const N: usize = 11;
impl Solution {
    pub fn find_duplicated_numbers(a: [i32; N]) -> Result<Vec<i32>, bool> {
        let mut b: [i32; N] = [0; N];
        for v in &a {
            if *v < 0 || *v > N as i32 {
                return Err(false);
            };
            b[*v as usize] += 1;
        }
        let mut res = vec![];
        let mut i: i32 = 0;
        let mut v: i32 = 0;
        loop {
            if i >= N as i32 {
                break;
            }
            v = b[i as usize];
            if v > 1 {
                res.push(i);
            }
            i += 1;
        }
        if res.len() == 0 {
            return Err(false);
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_duplicated() {
        let a = [1, 3, 9, 6, 7, 6, 7, 8, 0, 2, 2];
        let expect = vec![2, 6, 7];
        match Solution::find_duplicated_numbers(a) {
            Ok(vec) => assert_eq!(vec, expect),
            Err(_) => panic!("Failed"),
        }
    }
}
