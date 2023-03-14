pub fn reverse_string(s: &mut Vec<char>) {
    let (mut start, mut end) = (0, s.len()-1);
    let mut tmp = s[start];
    while start < end {
        tmp = s[start];
        s[start] = s[end];
        s[end] = tmp;
        start += 1;
        end -= 1;
    }
}
