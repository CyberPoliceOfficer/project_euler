fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    
    return true;
}

fn main() {
    let target: u64 = 600_851_475_143;
    let target_sqrt: f64 = target as f64;
    let mut i = target_sqrt.sqrt() as u64;

    println!("{}", i);

    while i > 0 {
        if target%i == 0 && is_prime(i)
        {
            break;
        }
        i -= 1;
    }
    println!("{}", i);
}
