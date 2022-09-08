#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Conversion(::std::num::ParseIntError);
        }

        errors {
            InvalidString(v: String) {
                display("Bad string provided ({})", v)
            }
        }
    }
}
use errors::*;

fn main() {

    /* This is a more complicated example - the error-chain crate provides macros for creating a
     *    new error struct with the Display, Debug, and Error traits implemented. It also has a
     *    From operator to convert strings into errors.
     */
    println!("Basic error throwing");
    for v in ["23", "x", ""] {
        match string_to_integer(&v) {
            Err(e) => println!("  {}", e),
            _ => ()
        }
    }

    /* That works, but doesn't really show the power of error handling. It is better to leverage
     *    the `e.kind()` function, which returns an enum of error types. This allows us to `match`
     *    against a number of expected error types, similar to how a try/catch block would work.
     * By default, error-chain implements ErrorKind::Msg, but we can add more.
     */
    println!("Matching on error kind");
    for v in ["23", "x", "", "a"] {
        if let Err(e) = string_to_integer_custom(&v) {
            match e.kind() {
                &ErrorKind::Conversion(ref s) => println!("  Conversion error: {}", s),
                &ErrorKind::InvalidString(ref s) => println!("  String content error: {}", s),
                &ErrorKind::Msg(ref s) => println!("  Default error: {}", s),
                _ => ()
            }
        }
    }
}

fn string_to_integer(input_string: &str) -> Result<i32> {
    if input_string.len() == 0 {
        bail!("empty string");
    }
    Ok(input_string.trim().parse()?)
}

fn string_to_integer_custom(input_string: &str) -> Result<i32> {

    if input_string.len() == 0 {
        bail!(ErrorKind::InvalidString("empty string".to_string()));
    }

    // Random error state, jus to show the default `Msg` type
    if input_string == "a" {
        bail!("Not an 'a'!");
    }

    let int_value = input_string.trim().parse::<i32>()?;
    Ok(int_value)
}
