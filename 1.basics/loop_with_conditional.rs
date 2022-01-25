fn main() {

    let divisor = 3;

    for i in 0..10 {

        let modulus =  i % divisor;

        if modulus == 0 {
            println!("Value {} is divisible by {}", i, divisor);
        } else {
            println!("Value {} is not divisible (remainder = {})", i, modulus);
        }
    }
}