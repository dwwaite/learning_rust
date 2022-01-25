fn main() {

    println!("This does the same thing as `loop_with_conditionals`, but written in a different way...");
    println!("");

    let divisor = 3;

    for i in 0..10 {

        let is_divisible = if i % divisor == 0 {"divisible"} else {"not divisible"};
        println!("Value {} is {} by {}", i, is_divisible, divisor);

    }
}