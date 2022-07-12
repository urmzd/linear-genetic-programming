fn main() {
    let mut x = vec![4, 3, 3, 4, 5];
    x.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("{:?}", x);
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", x);
}
