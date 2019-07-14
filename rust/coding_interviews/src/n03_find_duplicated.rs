pub struct Solution;
const N: usize = 10;
#[allow(dead_code)]
impl Solution {
    pub fn find_duplicated_numbers(a: [i32; N]) -> (i32, bool) {
        let mut a = a;
        let mut b: [i32; N] = [0; N];
        for v in &a {
            if *v < 0 || *v > N as i32 {
                return (0, false);
            };
        }
        let mut i: i32 = 0;
        loop {
            if i >= N as i32 {
                break;
            }
            while a[i as usize] != i {
                if a[i as usize] == a[a[i as usize] as usize] {
                    return (a[i as usize], true);
                }
                let temp = a[i as usize];
                a[i as usize] = a[temp as usize];
                a[temp as usize] = temp;
            }
            i += 1;
        }
        (0, false)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_duplicated() {
        let a = [1, 3, 9, 6, 7, 6, 7, 8, 0, 2];
        let (res, ok) = Solution::find_duplicated_numbers(a);
        assert!(ok);
        match res {
            7 => (),
            6 => (),
            _ => panic!("err {}", res),
        }
    }
}
