#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let length = s.len();
        let num_rows = num_rows as isize;
        let cycle = (num_rows - 1) * 2;

        if num_rows < 1 || num_rows > 1000 {
            panic!("numRows must be between 1 and 1000")
        }
        if length <= 1 || num_rows == 1 {
            return s;
        }

        let mut temp: HashMap<usize, String> = HashMap::new();

        for (idx, c) in s.chars().enumerate() {
            let idx = idx as isize;
            let n = idx % cycle;
            let key = if n >= num_rows { cycle - n } else { n } as usize;
            temp.entry(key)
                .or_insert_with(|| String::with_capacity(length / num_rows as usize))
                .push(c);
        }

        (0..num_rows as usize)
            .flat_map(|i| temp.get(&i))
            .map(|l| l.to_owned())
            .reduce(|s1, s2| s1 + &s2)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(Solution::convert(String::from(""), 1), String::from(""));
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
        assert_eq!(Solution::convert(String::from("B"), 1), String::from("B"));
    }

    #[test]
    #[should_panic(expected = "numRows must be between 1 and 1000")]
    fn error_handling() {
        assert_eq!(Solution::convert(String::from(""), 0), String::from(""));
        assert_eq!(
            Solution::convert(String::from("A"), 2000),
            String::from("A")
        );
        assert_eq!(
            Solution::convert(String::from("B"), -1000),
            String::from("B")
        );
    }

    #[test]
    fn input_order_is_preserved_when_number_of_rows_is_one() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
        assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"));
        assert_eq!(
            Solution::convert(String::from("ABC"), 1),
            String::from("ABC")
        );
        assert_eq!(
            Solution::convert(String::from("ABCD"), 1),
            String::from("ABCD")
        );
    }

    #[test]
    fn input_order_is_preserved_when_number_of_rows_is_greater_than_length() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
        assert_eq!(Solution::convert(String::from("AB"), 2), String::from("AB"));
        assert_eq!(
            Solution::convert(String::from("ABC"), 3),
            String::from("ABC")
        );
        assert_eq!(
            Solution::convert(String::from("ABCD"), 4),
            String::from("ABCD")
        );
    }

    #[test]
    fn two_rows_len3() {
        assert_eq!(
            Solution::convert(String::from("ABC"), 2),
            String::from("ACB")
        );
    }

    #[test]
    fn two_rows_len4() {
        assert_eq!(
            Solution::convert(String::from("ABCD"), 2),
            String::from("ACBD")
        );
    }

    #[test]
    fn two_rows_len5() {
        assert_eq!(
            Solution::convert(String::from("ABCDE"), 2),
            String::from("ACEBD")
        );
    }

    #[test]
    fn two_rows_len6() {
        assert_eq!(
            Solution::convert(String::from("ABCDEF"), 2),
            String::from("ACEBDF")
        );
    }

    #[test]
    fn three_rows_len4() {
        assert_eq!(
            Solution::convert(String::from("ABCD"), 3),
            String::from("ABDC")
        );
    }

    #[test]
    fn three_rows_len5() {
        assert_eq!(
            Solution::convert(String::from("ABCDE"), 3),
            String::from("AEBDC")
        );
    }

    #[test]
    fn three_rows_len6() {
        assert_eq!(
            Solution::convert(String::from("ABCDEF"), 3),
            String::from("AEBDFC")
        );
    }

    #[test]
    fn three_rows_len7() {
        assert_eq!(
            Solution::convert(String::from("ABCDEFG"), 3),
            String::from("AEBDFCG")
        );
    }
}
