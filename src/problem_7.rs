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
    let target: u64 = 10001;
    let mut prime_count: u64 = 0;
    let mut i: u64 = 0;

    while true {
        if is_prime(i)
        {
            prime_count += 1;
            if prime_count == target
            {
                break;
            }
        }
        i += 1;
    }
    println!("{}", i);
}
