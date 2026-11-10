pub fn count_characters(s: &str) -> u32 {
    // Count the number of characters in the string
    let mut cnt = 0;
    
    for c in s.chars() {
        cnt += 1;
    }

    cnt
    // CODE: unimplemented!()
}
