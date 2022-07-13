use ndarray::{s, Array, Array1};

fn main() {
    let mut x = Array1::from(vec![1; 10]);

    let mut y = Array::uninit((100, 10));

    (0..100).for_each(|g| {
        x.assign_to(y.slice_mut(s![g, ..]));
    });

    let v = unsafe { y.assume_init() };
    println!("{:?}", v);
}
