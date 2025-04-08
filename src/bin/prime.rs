fn main() {
    let _primes = prime_number(5);
    println!("{:?}", _primes)
}

fn prime_number(n: i32) -> Vec<i32> {
    let mut primes = Vec::new();
    for i in 2..n + 1 {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    primes
}
