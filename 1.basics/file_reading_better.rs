use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn main() {

    /* The last version was fine - it does exactly what I want, but it's not the best way to write
     *    the code. The error handling is...functional, but it becomes a pathway to just crash rather
     *    than elegantly handle the error.
     * Enter the `IResult` type - this is like Option, but rather than Some/None, Result wraps Ok and Err.
     *    Fairly obvious what these are...
     */
    let my_file = env::args().nth(1).expect("Please supply a file");

    /* This function does everything we need, but it's a bit gnarly. There are some awesome simplifications
     *    that can be applied. Hence the re-write...
     */
    //let file_content = _read_file_contents(&my_file);
    let file_content = read_file_contents_better(&my_file);

    if file_content.is_ok() {
        parse_file(&my_file, &file_content.unwrap());
    } else {
        println!("There was an error reading file '{}'", my_file);
    }

}

fn _read_file_contents(file_path: &str)-> Result<String, io::Error> {

    /* The return type here is a Result wrapper that wraps either a String (the content)
     *    or an Error type
     */

    // Open the file handle using the match/assignment pattern that Rust likes
    let mut file_handle = match File::open(&file_path) {

        // If anything goes wrong, return the error wrapped in the Result type
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    /* If the code above worked, we have a valid file read. Process it.
     * Same function to read as last time, but this time we wrap the returned value in the Result
     *    type as well. If everything goes fine, the `file_content` variable is returned wrapped in
     *    a Result, but if it fails the Result instead wraps an error.
     */

    let mut file_content = String::new();
    match file_handle.read_to_string(&mut file_content) {
        Ok(_) => Ok(file_content),
        Err(e) => Err(e),

        // Note no semi-colon below - implicit return
    }
}

fn read_file_contents_better(file_path: &str)-> io::Result<String> {

    /* The return type here is the same as the first function, it is simply an alias for the same call.
     * We can also use a much simpler code syntax by invoking the `?` operator.
     *    This basically short-cuts the whole match pattern - if there's an error, it is wrapped and
     *    returned in the Result type.
     * If we make it to the end of the code block, then we just need to wrap the final value in the
     *    Result operator and return it.
     */

    let mut file_handle = File::open(&file_path)?;

    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn parse_file(file_path: &str, file_content: &str) {

    // Really just a wrapper for the code of the previous file handling exercise...
    println!("File '{}' contains the following content:", file_path);

    let lines = split_string_newlines(&file_content);

    let mut i: i32 = 1;
    for value in lines {
        println!("Line {}: '{}'", i, value);
        i += 1;
    }
}

fn split_string_newlines(input_string: &str) -> Vec<&str> {

    // Now that I've implemented this myself, I'll use the built-in method like a real programmer!
    let mut file_lines: Vec<&str> = input_string.split("\n").collect();

    // Just remove the terminal value if it's empty...
    if file_lines[ file_lines.len() - 1 ] == "" {
        file_lines.pop();
    }

    file_lines
}