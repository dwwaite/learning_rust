use std::num::ParseIntError;

fn main() {

    /* Say we have a vector of strings to parse into numeric values. The python way to do this would
     *    be to use a try/catch box around the conversion and expect the conversion error for fail
     *    cases, but rust doesn't work that way.
     * The rust way to do this is of course to wrap the outcome of each conversion attempt in a
     *    Result<> and then evaluate afterwards. This can be a bit clunky and verbose to code out
     *    yourself, but there is intelligence in a lot of the parsing functionality to handle this.
     * However, there is another clunky issue:
     */

    let values = vec![
        "10".to_string(),
        "20".to_string(),
        "30".to_string(),
        "40".to_string()
    ];

    // This approach automatically wraps each value. That means that when we invoke it, each value in
    //    the new vector would have to be unwrapped manually.
    let converted_values = collect_wrapped_values(&values);
    println!("{:?}", converted_values);
    for converted_value in &converted_values {
        match converted_value {
            Ok(x) => println!("  {}", x),
            _ => println!("  Failed!")
        };
    }

    /* There is value in this approach, as it means that if some values fail to cast we can still process
     *    the ones that succeed. However, in most use cases I would have we would probably need to abort
     *    if anything wasn't accepted, so this is a bit verbose.
     * Instead, we could specify the we want the entire collected vector to be wrapped inside the result,
     *    so that if it works, easy to work with, but if not then the entire thing is an error.
     */
    let converted_values = collect_unwrapped_values(&values);
    println!("{:?}", converted_values);
    if converted_values.is_ok() {
        for converted_value in &converted_values.unwrap() {
            println!("  {:?}", converted_value);
        }
    }

    /* Both of these are useful approaches - for my work probably the later makes more sense, but it's good
     *    to see the flexibility in the `.collect()` function as it's something that I'm going to be using
     *    a lot as I dive into rust. 
     */
}

fn collect_wrapped_values(values: &Vec<String>) -> Vec<Result<i32, ParseIntError>> {

    let iter = values.iter().map(|s| s.parse::<i32>());

    // Lazy typing here - rustc knows what it's getting by compiling the previous line,
    //    but I had to work out the return type of the function through a compile error.
    let converted: Vec<_> = iter.collect();
    converted
}

fn collect_unwrapped_values(values: &Vec<String>) -> Result<Vec<i32>, ParseIntError> {

    let iter = values.iter().map(|s| s.parse::<i32>());

    // Have to shuffle around the return type a little for this...
    let converted: Result<Vec<_>,_> = iter.collect();
    converted
}
