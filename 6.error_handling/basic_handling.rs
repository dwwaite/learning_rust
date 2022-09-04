use std::{env,fmt};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::num::ParseFloatError;

fn main() {

    /* Handling a Result<> is quite easy when there is only a single source of the error in the
     *    function that calls it, but realistically there are multiple sources of errors for
     *    many cases.
     * The best example is proably file reading - there are at least two cases which can happen
     *    for any file read - a failure to open the file (path or permission) and then a failure
     *    to parse the contents. How to do we handle these different cases?
     */

    // Determine set the file we are trying to read, with a default case for when nothing is provded.
    let file_path = match env::args().nth(1) {
        Some(x) => x,
        None => "R_multiflora.fna".to_string()
    };

    // File read example (see function for notes)
    let contents = basic_read(&file_path);
    match contents {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e)
        }
    }

    // It is likely that in any moderately complex programme, we will have to create custom Error
    //    types. This is quite easy to achieve.
    let will_raise = match env::args().nth(1) {
        Some(_) => true,
        None => false
    };

    match raise_custom_error(will_raise) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e)
        }
    }

    // Finally, we can also cause our custom errors to capture and consume a different error type.
    //    Here, the custom error now overwrites a float conversion error via the From operator.
    for s in ["30", "30.5", "a"] {
        match parse_float(s) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{:?}", e)
            }
        }
    }

}

fn basic_read(file_path: &str) -> Result<String,Box<dyn Error>> {

    /* When there are multiple cases to return, we can't easily handle the return type as the 
     *    Result<> container explicitly states the error type it is returning.
     * How do we we wrap into a generic content in rust? With a Box. Fortuantely, all errors in
     *    implement the `std:error:Error` trait so we can easily specify a Box to hold any error. 
     */

     let mut file_handle = File::open(file_path)?; // Can't open.
    
     let mut file_contents = String::new();
     file_handle.read_to_string(&mut file_contents)?; // Can't read as string.

     Ok(file_contents.trim().parse()?)
}

fn raise_custom_error(raise_error: bool) -> Result<(), CustomError> {
    if raise_error {
        Err(CustomError::new("You wanted an error?"))
    } else {
        Ok(())
    }
}

fn parse_float(input_value: &str) -> Result<f64, CustomError> {
    let f_value = input_value.parse()?;
    Ok(f_value)
}

//region Custom error

#[derive(Debug)]
struct CustomError {
    details: String
}

impl CustomError {
    fn new(error_message: &str) -> CustomError {
        CustomError{details: error_message.to_string()}
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseFloatError> for CustomError {
    fn from(original_error: ParseFloatError) -> Self {
        CustomError::new(&original_error.to_string())
    }
}

//endregion
