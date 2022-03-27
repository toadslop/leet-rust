pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for char in s.chars() {
        match is_opener(char) {
            true => stack.push(map_open_to_close(char).unwrap()),
            false => {
                if !(stack.pop().unwrap_or('*') == char) {
                    return false;
                };
            }
        }
    }

    stack.len() == 0
}

fn map_open_to_close(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '{' => Some('}'),
        '[' => Some(']'),
        _ => None,
    }
}

fn is_opener(c: char) -> bool {
    match map_open_to_close(c) {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let result = is_valid(String::from("()"));
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let result = is_valid(String::from("()[]{}"));
        assert_eq!(result, true);
    }

    #[test]
    fn example_3() {
        let result = is_valid(String::from("(]"));
        assert_eq!(result, false);
    }

    #[test]
    fn example_4() {
        let result = is_valid(String::from("({})[{()}]{}"));
        assert_eq!(result, true);
    }

    #[test]
    fn example_5() {
        let result = is_valid(String::from("({})[{(}]{}"));
        assert_eq!(result, false);
    }

    #[test]
    fn example_6() {
        let result = is_valid(String::from("["));
        assert_eq!(result, false);
    }

    #[test]
    fn example_7() {
        let result = is_valid(String::from("]"));
        assert_eq!(result, false);
    }
}
