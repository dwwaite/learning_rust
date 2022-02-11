// ./command_line_capture 1 2 3
use std::env;

fn main() {

    /* Fairly easy to capture them via an iterator.
     * Results are returned as strings, the executing programme is in position 0
     */
    for arg in env::args() {
        println!("Iterable value: {}", arg);
    }

    /* Instead, can access by position. This is better when we expect a particular
     *    order to the incoming arguments. For example, we might want to parse
     *    some as numeric values and keep others as strings.
     */
    let first = env::args().nth(1);
    if first.is_some() {
        println!("Unwrapped value: {}", first.unwrap());
    }

     /* Easy enough. However, it's a fair few lines to just grab a value. There are
      *    two ways to simplify this:
      */
    let first = env::args().nth(1).expect("Please supply an argument!"); 
    let first_int: i32 = first.parse().expect("Cannot convert value to integer!");
    println!("Cast value: {}", first_int);

}