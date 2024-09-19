fn main() {
    let n = 100;
    let a = n * (n + 1) * (2*n + 1)/6;
    let b = n * (n + 1)/2;
    println!("{}", b*b - a);
}
