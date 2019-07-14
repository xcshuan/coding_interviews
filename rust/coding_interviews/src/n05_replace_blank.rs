use std::fs::copy;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn replace_blank(s: &mut str) -> String {
        let mut original_len = s.len();
        let res = String::new();
        let res = s.replace(" ", "%20");
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_replace_blank() {
        let mut a = String::from("www.google.com/a b c d e");
        let mut a = a.as_mut_str();
        let s = Solution::replace_blank(a);
        assert_eq!(s, String::from("www.google.com/a%20b%20c%20d%20e"));
    }
}
