pub fn length_of_longest_substring(s: String) -> i32 {
    let mut counter = vec![0; 255];
    let mut sub = String::new();
    let mut max = 0;

    for c in s.chars() {
        sub.push(c);
        counter[c as usize] += 1;
        if counter[c as usize] == 1 {
            if sub.len() > max {
                //println!("max={}", sub);
                max = sub.len();
            }
        } else {
            let mut cut = 1;
            for sc in sub.chars() {
                if sc == c {
                    break;
                }
                counter[sc as usize] = 0;
                cut += 1;
            }
            counter[c as usize] = 1;
            sub = sub[cut..].to_string();
        }
    }
    max as i32
}

pub fn get_counter(s: &[u8]) -> Vec<i32> {
    let mut counter = vec![0; 255];
    for c in s.iter() {
        counter[*c as usize] += 1;
    }
    counter
}

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let slen = s1.len();
    let s1: &[u8] = s1.as_bytes();
    let s2: &[u8] = s2.as_bytes();

    let c1 = get_counter(&s1);
    let mut c2 = get_counter(&s2[0..s1.len()]);
    if c1 == c2 {
        return true;
    }

    for i in 1..=(s2.len() - s1.len()) {
        c2[s2[i - 1] as usize] -= 1; // remove past ch
        c2[s2[i + slen - 1] as usize] += 1; // add next ch
        if c1 == c2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest() {
        let mut d3 = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring(d3), 3);
        let d1 = "bbbbbb".to_string();
        assert_eq!(length_of_longest_substring(d1), 1);
        d3 = "pwwkew".to_string();
        assert_eq!(length_of_longest_substring(d3), 3);
        d3 = "dvdf".to_string();
        assert_eq!(length_of_longest_substring(d3), 3);
    }

    #[test]
    fn test_inclusion() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert!(check_inclusion(s1, s2));
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert!(!check_inclusion(s1, s2));
    }
}
