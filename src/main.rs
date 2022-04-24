mod list;
mod qsort;
mod stack;

fn main() {
    let mut data = vec![10, 10, 11, 12, 13, 10, 10, 16, 1, 2, 10, 10, 5, 10];
    let hi = data.len() - 1;
    qsort::qsort_threeway(&mut data, 0, hi);
    println!("{:?}", data);
}
