
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[macro_use]
extern crate error_chain;

// This tim we're not linking any existing errors, and also creating no new ErrorKind instances
mod errors {
    error_chain!{
    }

}
use errors::*;

fn main() {

    /* Awkwardly, for this approach to work in the way I want it to (multiple error possibilities
     *    with a single statement to examine them) I need to place all my error-prone operations
     *    into a single function.
     */
    if let Err(e) = parse_file() {

        println!("Error raised: {} ({})", e, e.kind());

        // Examine each step in the chain
        for e_link in e.iter().enumerate() {

            // Destructure the enumerate tuple to get the individual components
            let (i, e_msg) = e_link;
            println!("  Step {}: {}", i, e_msg);

        }

    }

}

fn parse_file() -> Result<()> {
    // Scoping args() within the function, just retrieving the expected file name position.
    use std::env::args;

    // For this instance, it's just a standard error, no chaining.
    let file_path = args()
        .skip(1) // First instance is self
        .next()  // Hop to the second instance
        .ok_or(
            Error::from("No filename provided!")
        )?;

    // Attempt the standard open+? combination, but this time suffix the `.chain_err()` function.
    // This takes a closure with the error message which will wrap around the base error.
    let file_handle = File::open(&file_path).chain_err(|| "Unable to open file!")?;

    // Final segment of the code, attempt to read the file, throwing another chained error on fail.
    let mut i: u32 = 1;
    for line in BufReader::new(file_handle).lines() {

        // Prepare the error message, in case it is needed
        let mut err_msg = "Cannot read file line ".to_string();
        err_msg += &i.to_string();

        // Not actually doing anything with the line, this is just to make the read attempt happen.
        let _line = line.chain_err(|| err_msg)?;

        i += 1;
    }

    Ok(())
}