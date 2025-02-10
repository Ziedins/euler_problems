fn main() {
    
    let mut largest_palindrome_number : u64 = 0;

    for i in 100..1000
    {
        for t in 100..1000 {
            let mut number: u64 = 0;
            number = i * t;
            let mut remainder : u64 = number;
            let mut digits : Vec<u8> = Vec::new();
            while remainder > 0 {
               digits.push((remainder % 10).try_into().unwrap());
               remainder = remainder / 10;
            }
            let digits_original = digits.clone();
            digits.reverse();
            if digits == digits_original && number > largest_palindrome_number {
                println!("made of these factors : {i:?}, {t:?}" );
                largest_palindrome_number = number;
            }
        }
    }   

    println!("Largest Palindrome number : {largest_palindrome_number:?}");

}
