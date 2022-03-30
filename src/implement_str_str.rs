#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    const IDX_TOO_BIG: &'static str = "Index was too big to convert to i32";
    const NOT_FOUND: i32 = -1;

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut i: usize = 0;
        if needle.is_empty() {
            return i.try_into().expect(Self::IDX_TOO_BIG);
        }

        while let Some(slice) = haystack.get(i..i + needle.len()) {
            if slice == needle {
                return i.try_into().expect(Self::IDX_TOO_BIG);
            }
            i += 1;
        }

        Self::NOT_FOUND
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let haystack = String::from("hello");
        let needle = String::from("ll");

        let result = Solution::str_str(haystack, needle);
        let expected = 2;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() -> () {
        let haystack = String::from("aaaaa");
        let needle = String::from("bba");

        let result = Solution::str_str(haystack, needle);
        let expected = -1;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_3() -> () {
        let haystack = String::from("");
        let needle = String::from("");

        let result = Solution::str_str(haystack, needle);
        let expected = 0;

        assert_eq!(result, expected);
    }
}
