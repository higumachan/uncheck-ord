use unwrap_ord::UnwrapOrd;

fn main() {
    let v = [1.0, 3.0, 2.0];
    let mut v: Vec<_> = v.iter().copied().map(|x| UnwrapOrd(x)).collect();

    v.sort();

    println!("{:?}", v);  // [UncheckOrd(1.0), UncheckOrd(2.0), UncheckOrd(3.0)]
}