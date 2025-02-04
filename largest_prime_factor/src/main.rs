use std::io;

fn main() {
    loop {
        println!("Number to get factors from");
        let mut input_number = String::new();
        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read input");

        let number : u64;
        number = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue
            }
        };
        let mut largest_prime_factor : u64 = 0;

        for i in 2..number.isqrt() {
            if number % i == 0 {
                if is_prime(i) {
                    if i > largest_prime_factor {
                        largest_prime_factor = i;
                    }
                    println!("Prime factor : {i:?} ");
                }
            }
        }
        if largest_prime_factor == 0 {
            largest_prime_factor = number;
        }
        println!("Largest Prime factor : {largest_prime_factor:?}");
    }
}

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    
    for i in 2..number {
        if number % i == 0 {
            return false;
        }

    }

    return true;
}
