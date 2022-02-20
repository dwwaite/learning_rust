use std::env;
use std::fs::File;
use std::io::Read;

fn main() {

    /* When working with file reading, it's important to get the error handling sorted up front.
     * One simple and rust-y way to do this is with the `expect()` function.
     */
    let my_file = env::args().nth(1).expect("Please supply a file");

    /* However, this isn't all that informative. The exact error/message returned when no file
     *    is provided is:
     * ```thread 'main' panicked at 'Please supply a file', 1.basics/file_reading.rs:10:38
     *    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace```
     * Informative, but not much more user friendly than a stack trace.
     */
    
    /* Once we actually have a file handle, we then also need to confirm that we are able to 
     *    read it as well...
     * ```
     * touch blah.txt
     * chmod -r blah.txt
     * ```
     */
    let mut error_message = String::new();
    error_message += "Unable to open the file '";
    error_message += &my_file.to_string();
    error_message.push_str("'");

    let mut input_file = File::open(&my_file).expect(&error_message);

    // Finally, we actually read the file:
    println!("File '{}' contains the following content:", my_file);

    let mut file_content = String::new();
    input_file.read_to_string(&mut file_content).expect("Can't read the file");

    let lines = split_string_newlines(&file_content);

    let mut i: i32 = 1;
    for value in lines {
        println!("Line {}: '{}'", i, value);
        i += 1;
    }

}

fn split_string_newlines(input_string: &str) -> Vec<String> {

    /* Since I'm still in absolute rookie mode, start by just iterating the characters of the string
     *    and operating on the newline characters.
     * Need to two variables here:
     *    1. A Vector of strings to carry the line entries
     *    2. A fresh string, to build upon as the content of the file is iterated char by char
     */
    
    let mut file_lines = Vec::new();
    let mut current_string = String::new();

    for c in input_string.chars() {
        if c == '\n' {

            file_lines.push(current_string);
            current_string = String::new();

        } else {
            current_string += &c.to_string();
        }
    }

    /* If there is content on the last line, add it too.
     *    -> But if it's an empty newline, ignore it...
     */
    if current_string != "" {
        file_lines.push(current_string);
    }

    file_lines
}