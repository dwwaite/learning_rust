#[macro_use]
extern crate simple_error;
use std::error::Error;

fn main() {

    /* This is a simple package that basically lets you create errors based on a string
     *    description. Rather than raise loads of different error types, this way you are
     *    basically working with a single generic error type, which has an inforamtive text
     *    description.
     */
    println!("{:?}", string_to_integer("23"));
    println!("{:?}", string_to_integer("x"));
    println!("{:?}", string_to_integer(""));
}

//type BoxResult<T> = Result<T,Box<dyn Error>>;
fn string_to_integer(input_string: &str) -> Result<i32, Box<dyn Error>> {

    /* The original example created a new type called `BoxResult` to simplify the return process,
     *    but at my level of experience I would rather keep things a bit more basic.
     */

    if input_string.len() == 0 {

        // Macro which is essentially `return SimpleError::new(s).into();`
        bail!("empty string");
    }

    // Use of the `?` operator to return the error on the parse failure.
    Ok(input_string.trim().parse()?)
}