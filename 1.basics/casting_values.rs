fn main() {

    println!("This is the same idea as `mutability`, but this time adding the int counter to a float total...");

    // Can implicitly determine type here, but cannot for a function (see below)
    let mut sum = 0.5;

    for i in 0..10 {

        sum += i as f64;
        println!("Adding value {}, current total is {}", i, sum);

    }
    println!("Final value is {}", sum);

    let sqr_sum = sqr_value(sum);
    println!("Squared value is {}", sqr_sum);
}

fn sqr_value(x: f64) -> f64 {

    /* Return statement is implcitly the last line of the function, so the statement
     *    return x * x;
     * is the same as what is below.
     *
     * NOTE - there is no semi-colon on the implicit return line! 
     */
     x * x

}