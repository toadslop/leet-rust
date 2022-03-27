pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs = strs.clone();
    strs.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut prefix = strs.get(0).expect("Failed to get first member.").to_owned();
    for item in strs[1..].iter() {
        let mut slice = prefix.as_str();
        let mut end = slice.len();
        while !item.starts_with(&slice) && end > 0 {
            end = end - 1;
            slice = &slice[0..end];
        }
        prefix = String::from(slice);
    }

    return prefix;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
