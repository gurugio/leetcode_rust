use std::cmp::Ordering;

fn partition<T: Ord>(a: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi;
    loop {
        loop {
            if let Ordering::Greater = a[i].cmp(&a[lo]) {
                break;
            } else {
                if i == hi {
                    break;
                }
                i += 1;
            }
        }
        loop {
            if let Ordering::Less = a[j].cmp(&a[lo]) {
                break;
            } else {
                if j == lo {
                    break;
                }
                j -= 1;
            }
        }

        if i >= j {
            break;
        }

        a.swap(i, j);
    }
    a.swap(lo, j);
    return j;
}

pub fn qsort<T: Ord>(a: &mut Vec<T>) {
    let hi = a.len() - 1;
    qsort_internal(a, 0, hi);
}

fn qsort_internal<T: Ord>(a: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let j = partition(a, lo, hi);
    if j > 0 {
        qsort_internal(a, lo, j - 1);
    }
    if j < (a.len() - 1) {
        qsort_internal(a, j + 1, hi);
    }
}

pub fn select<T: Ord + Clone>(a: &mut Vec<T>, k: usize) -> T {
    let mut lo = 0;
    let mut hi = a.len() - 1;
    while hi > lo {
        let j = partition(a, lo, hi);
        if j < k {
            lo = j + 1;
        } else if j > k {
            hi = j - 1;
        } else {
            return a[k].clone();
        }
    }
    return a[k].clone();
}

pub fn qsort_threeway<T>(a: &mut Vec<T>, lo: usize, hi: usize)
where
    T: Ord + std::fmt::Debug + Clone,
{
    if hi <= lo {
        return;
    }
    let mut lt = lo;
    let mut gt = hi;
    let mut i = lo;
    let v = a[lo].clone();

    while i <= gt {
        match a[i].cmp(&v) {
            Ordering::Less => {
                a.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                a.swap(i, gt);
                gt -= 1;
            }
            Ordering::Equal => i += 1,
        }
    }

    if lt > 0 {
        qsort_threeway(a, lo, lt - 1);
    }
    if gt + 1 < a.len() {
        qsort_threeway(a, gt + 1, hi);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partition() {
        let mut data = vec![10, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6];
        let mut hi = data.len() - 1;
        assert_eq!(7, partition(&mut data, 0, hi));
        assert_eq!(vec![1, 10, 6, 5, 4, 3, 2, 10, 16, 15, 14, 13, 12, 11], data);

        data = vec![1, 2, 3, 4];
        hi = data.len() - 1;
        let j = partition(&mut data, 0, hi);
        println!("{:?}, {}", data, j);
    }

    #[test]
    fn test_qsort() {
        let mut data = vec![10, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6];
        qsort(&mut data);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 10, 10, 11, 12, 13, 14, 15, 16], data);
    }

    #[test]
    fn test_select() {
        let mut data = vec![10, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6];
        assert_eq!(select(&mut data, 0), 1);
        assert_eq!(select(&mut data, 1), 2);
        assert_eq!(select(&mut data, 2), 3);
        assert_eq!(select(&mut data, 3), 4);
        assert_eq!(select(&mut data, 4), 5);
        assert_eq!(select(&mut data, 5), 6);
        assert_eq!(select(&mut data, 6), 10);
    }

    #[test]
    fn test_threeway() {
        let mut data = vec![10, 10, 11, 12, 13, 10, 10, 16, 1, 2, 10, 10, 5, 10];
        let hi = data.len() - 1;
        qsort_threeway(&mut data, 0, hi);
        assert_eq!(
            vec![1, 2, 5, 10, 10, 10, 10, 10, 10, 10, 11, 12, 13, 16],
            data
        );
    }
}
