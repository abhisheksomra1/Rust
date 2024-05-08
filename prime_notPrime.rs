fn main() {
    let num = 17;
    let mut is_prime = true;

    if num <= 1 {
        is_prime = false;
    } else if num == 2 {
        is_prime = true;
    } else if num % 2 == 0 {
        is_prime = false;
    } else {
        let mut i = 3;
        while i * i <= num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
            i += 2;
        }
    }

    if is_prime {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
