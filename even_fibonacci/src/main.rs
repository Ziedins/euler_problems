fn main() {
    let mut current_fibonacci: u32 = 2;
    let mut previous_fibonacci: u32 = 1;
    let mut even_sum: u32 = 0;
    while current_fibonacci < 4000000  {
        if current_fibonacci % 2 == 0 {
            even_sum += current_fibonacci;
        }

        let to_be: u32 = current_fibonacci;
        current_fibonacci = current_fibonacci + previous_fibonacci;
        previous_fibonacci = to_be;
    }

    println!("Fibonacci even number sum: {even_sum:?}");
}
