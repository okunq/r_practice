fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    for i in 2..=(*n as f64).sqrt() as u32 {
        if *n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 100, 10007];
    for n in numbers {
        println!("{n} is prime? {}", is_prime(&n));
    }
}