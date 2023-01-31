use std::collections::BTreeMap;

#[cfg(test)]
struct Solution {}


#[cfg(test)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let length = s.len();

        if num_rows < 1 || num_rows > 1000 { panic!("numRows must be between 1 and 1000") }
        if length == 0 { return s; }
// A0 B1 D2 C3
        let groups = (length - 1)/num_rows as usize + 1;
        let mut temp = BTreeMap::new();
        for (i, b) in s.bytes().enumerate() {
            
            let key = (i * groups) % length + i / num_rows as usize;
            println!("i: {}, num_rows: {}, groups: {}, length: {}, key: {}, char: {}", i, num_rows, groups, length, key, b as char);
            if temp.contains_key(&key) { break; } 
            temp.entry(key).or_insert(b);
        }
        String::from_utf8(temp.into_values().collect()).unwrap()
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
        assert_eq!(Solution::convert(String::from("A"), 2000), String::from("A"));
        assert_eq!(Solution::convert(String::from("B"), -1000), String::from("B"));
    }
    
    #[test]
    fn input_order_is_preserved_when_number_of_rows_is_one() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
        assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"));
        assert_eq!(Solution::convert(String::from("ABC"), 1), String::from("ABC"));
        assert_eq!(Solution::convert(String::from("ABCD"), 1), String::from("ABCD"));
    }
    
    #[test]
    fn input_order_is_preserved_when_number_of_rows_is_greater_than_length() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
        assert_eq!(Solution::convert(String::from("AB"), 2), String::from("AB"));
        assert_eq!(Solution::convert(String::from("ABC"), 3), String::from("ABC"));
        assert_eq!(Solution::convert(String::from("ABCD"), 4), String::from("ABCD"));
    }

    #[test]
    fn two_rows_len3() {
        assert_eq!(Solution::convert(String::from("ABC"), 2), String::from("ACB"));
    }

    #[test]
    fn two_rows_len4() {
        assert_eq!(Solution::convert(String::from("ABCD"), 2), String::from("ACBD"));
    }
    
    #[test]
    fn two_rows_len5() {
        assert_eq!(Solution::convert(String::from("ABCDE"), 2), String::from("ACEBD"));
    }
    
    #[test]
    fn two_rows_len6() {
        assert_eq!(Solution::convert(String::from("ABCDEF"), 2), String::from("ACEBDF"));
    }

    #[test]
    fn three_rows_len4() {
        assert_eq!(Solution::convert(String::from("ABCD"), 3), String::from("ABDC"));
    }
    
    #[test]
    fn three_rows_len5() {
        assert_eq!(Solution::convert(String::from("ABCDE"), 3), String::from("AEBDC"));
    }
    
    #[test]
    fn three_rows_len6() {
        assert_eq!(Solution::convert(String::from("ABCDEF"), 3), String::from("AEBDFC"));
    }
    
    #[test]
    fn three_rows_len7() {
        assert_eq!(Solution::convert(String::from("ABCDEFG"), 3), String::from("AEBDFCG"));
    }


}
