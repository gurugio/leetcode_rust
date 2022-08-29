fn isBadVersion(n: i32) -> bool {
    if n >= 1702766719 {
        return true;
    }
    false
}

pub fn first_bad_version(n: i32) -> i32 {
    let mut good = 1;
    let mut bad = n;
    let mut next; // = (left + right) / 2;
    loop {
        let t: u64 = good as u64 + bad as u64;
        next = (t / 2) as i32;
        if good == bad - 1 {
            return bad;
        }

        if isBadVersion(next) {
            bad = next;
        } else {
            good = next;
        }
    }
}
