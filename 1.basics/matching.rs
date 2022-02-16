// ./command_line_capture first second third
use std::env;

fn main() {

    /* Strictly speaking, this is not a part of argument capture, but combining the two gives
     *    a good method of demonstrating the technique.
     *
     * In Rust, a common approach for switch/case statements is to use the `match` pattern.
     */
    println!("");
    println!("Matching on arguments:");

    for arg in env::args() {
        match_arguments(&arg);
    }

    // Once we have done that, there is also the option to ignore the fail case:
    println!("");
    println!("Simplified matching:");

    let arg_value = env::args().nth(1).unwrap();
    if let Some(idx) = arg_value.find('i') {
        println!("A bit simpler, we have an `i` at index {} in argument '{}'", idx, arg_value);
    }

    

    // There is also an automatic fail case, when using match:
    println!("");
    println!("Matching with fallback case:");

    for i in &[10, 15, 20, 25, 30] {

        let message = match i {

            10 => "ten",
            20 => "twenty",
            _ => "something else"

        };
        println!("Value {} is parsed as {}", i, message);
    }

    /* This can also be done with ranges (...):
     *    NOTE: The syntax in the tutorial differs here - it has been updated in 2021+ Rust releases
     */

    println!("");
    println!("Matching with ranges:");

    for i in &[10, 15, 20, 25, 30] {

        let message = match i {

            10..=20 => "small",
            21..=29 => "moderate",
            _ => "large"

        };
        println!("Value {} is parsed as {}", i, message);
    }

}

fn match_arguments(input_arg: &str) {

    /* This is where Options become powerful - the result of `find()` is wrapped in a Some/None,
     *    and we can perform the match based on this.
     * This is basically the same as the unwrapping I have done previously with the pattern:
     *    let idx = input_arg.find('i');
     *    if idx.is_some() {
     *        println!(...);
     *    }
     * But this system is a bit nicer to look at...
     */
    match input_arg.find('i') {
        Some(idx) => {
            println!("For arg '{}' there is an `i` at index {}", input_arg, idx)
        },
        None => println!("For arg '{}', there is no `i` character.", {input_arg})
    };

}