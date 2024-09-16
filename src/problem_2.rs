fn main() {
    let mut a = 1;
    let mut b = 0;
    let mut i = 0;
    let mut sum = 0;

    while b < 4_000_000 {
        i = a + b;
        a = b;
        b = i;
        if i%2 == 0
        {
            sum += i;
        }

    }
    println!("{}", sum);
}
