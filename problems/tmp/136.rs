impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        
    let new_s: Vec<_> = s.to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<Vec<_>>();


    if new_s.is_empty() || new_s.len() == 1 {
            return true;
    }
        
        
    let mut i: usize = 0 as usize;
    let mut j: usize = new_s.len() - 1 as usize;


    while j >= i {
        if new_s[i] != new_s[j] {
            return false;
        }
        i += 1; j -= 1;
    }

    true
    }
}