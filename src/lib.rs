#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        match (s.is_ascii(), s.len(), num_rows) {
            (s, _, _) if !s => panic!("s must be ascii!"),
            (_, _, n) if n < 1 => panic!("num_rows must be at least 1"),
            (_, l, _) if l < 1 => panic!("s must have at least one char"),
            (_, _, n) if n > 1000 => panic!("num_rows must be at most 1000"),
            (_, l, _) if l > 1000 => panic!("s must be at most 1000 chars in length"),
            (_, l, n) if l == 1 || n == 1 => return s,
            (_, _, _) => (),
        }
        let mut rows = HashMap::new();
        let num_rows = num_rows as usize;
        let capacity = s.len() / num_rows;
        let cycle = (num_rows - 1) * 2;

        for (idx, ch) in s.chars().enumerate() {
            let dy = idx % cycle;
            let row = if dy >= num_rows { cycle - dy } else { dy };
            rows.entry(row)
                .or_insert_with(|| String::with_capacity(capacity))
                .push(ch);
        }

        (0..num_rows)
            .flat_map(|i| rows.get(&i))
            .map(|l| l.to_owned())
            .reduce(|s1, s2| s1 + &s2)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
        assert_eq!(Solution::convert(String::from("B"), 1), String::from("B"));
    }

    #[test]
    #[should_panic(expected = "s must be ascii!")]
    fn ascii_error_handling() {
        assert_eq!(Solution::convert(String::from("سلام"), 0), String::from(""));
    }

    #[test]
    #[should_panic(expected = "num_rows must be at least 1")]
    fn low_rows_error_handling() {
        assert_eq!(Solution::convert(String::from("ABC"), 0), String::from(""));
    }

    #[test]
    #[should_panic(expected = "num_rows must be at most 1000")]
    fn high_rows_error_handling() {
        assert_eq!(
            Solution::convert(String::from("ABC"), 9999),
            String::from("")
        );
    }

    #[test]
    #[should_panic(expected = "s must have at least one char")]
    fn empty_string_error_handling() {
        assert_eq!(Solution::convert(String::from(""), 3), String::from(""));
    }

    #[test]
    #[should_panic(expected = "s must be at most 1000 chars in length")]
    fn long_string_error_handling() {
        assert_eq!(
            Solution::convert("XXXXXCXXXX".repeat(101), 3),
            String::from("")
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
