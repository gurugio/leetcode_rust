use std::collections::HashMap;

pub fn fib(n: i32) -> i32 {
    let mut set = HashMap::new();
    set.insert(0, 0);
    set.insert(1, 1);
    fib_internal(n, &mut set)
}

pub fn fib_internal(n: i32, set: &mut HashMap<i32, i32>) -> i32 {
    if set.contains_key(&n) {
        println!("get {}-{}", n, *set.get(&n).unwrap());
        *set.get(&n).unwrap()
    } else {
        let v = fib_internal(n - 1, set) + fib_internal(n - 2, set);
        println!("insert {}-{}", n, v);
        set.insert(n, v);
        v
    }
}

pub fn tribonacci(n: i32) -> i32 {
    let mut set = HashMap::new();
    set.insert(0, 0);
    set.insert(1, 1);
    set.insert(2, 1);
    tri_internal(n, &mut set)
}

pub fn tri_internal(n: i32, set: &mut HashMap<i32, i32>) -> i32 {
    if set.contains_key(&n) {
        *set.get(&n).unwrap()
    } else {
        let v = tri_internal(n - 1, set) + tri_internal(n - 2, set) + tri_internal(n - 3, set);
        set.insert(n, v);
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(tribonacci(25), 1389537);
    }
}
