fn is_palindrome(num: u64) -> bool {
    let mut reverse = 0;
    let mut temp = num;
    while temp != 0 {
        reverse = (reverse * 10) + (temp % 10);
        temp = temp / 10;
    }
    return reverse == num;
}

fn main() {
    let mut palindrome = 0;
    let mut largest_palindrome = 0;
    'outer: for n in (100..999).rev() {
        for j in (100..n).rev() {
            palindrome = n*j;
            if is_palindrome(palindrome) && palindrome > largest_palindrome
            {
                largest_palindrome = palindrome
            }
        }
    }
    println!("{}", largest_palindrome);
}
