pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_in_2d_matrix(
        matrix: &Vec<Vec<i32>>,
        columns: usize,
        rows: usize,
        number: i32,
    ) -> Result<bool, &'static str> {
        if matrix.len() != rows {
            return Err("Size unmatched!");
        }
        for c in matrix {
            if c.len() != columns {
                return Err("Size unmatched!");
            }
        }
        let mut row = 0;
        let mut column = columns - 1;
        while row < rows && column >= 0 {
            if matrix[column][row] == number {
                return Ok(true);
            } else if matrix[column][row] >= number {
                column -= 1;
            } else {
                row += 1;
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_in_2d_matrix() {
        let a = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        assert!(Solution::find_in_2d_matrix(&a, 4, 4, 7).unwrap());

        assert!(!Solution::find_in_2d_matrix(&a, 4, 4, 100).unwrap())
    }
}
