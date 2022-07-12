fn main() {

    // Since we're getting back into using `match`, in the tutorials, there are
    //    some more things we can do with it.

    // Tuple matching examples
    println!("match_tuple_int:");
    match_tuple_int((0, "words!".to_string()));
    match_tuple_int((1, "more words!".to_string()));
    match_tuple_int((2, "no words!".to_string()));

    println!("match_tuple_string:");
    match_tuple_string((0, "words!".to_string()));
    match_tuple_string((1, "more words!".to_string()));
    match_tuple_string((2, "no words!".to_string()));

    // Turbofishing examples
    /* First up, we can use the `if let` syntax to match a tuple as well.
     *    Note how we're only comparing the String value of the tuple and
     *    that in this case the type of `s` is deduced by the compiler.
     * This isn't always the case...
     */
    let my_tuple = (2, "no words!".to_string());
    if let Some((_, ref s)) = Some(my_tuple) {
        assert_eq!(s, "no words!");
    }

    /* ...if there is not enough contextual information for the compiler to
     *    figure out the expected type, we can add a hint with the 'turbo
     *    fish' operation -> ::<t>
     */
    //if let Ok(n) = "42".parse() { => Error E0285, cannot infer type
    if let Ok(n) = "42".parse::<i32>() {
        assert_eq!(n, 42);
    }
}

//region Tuple matching

fn match_tuple_int(t: (i32, String)) {
    /* Here we take a tuple of form (i32, String), but only match on the first value.
     *    Importantly, here we are using the value of `s` but are not accessing it. To
     *    do so, we would need a different pattern.
     */
    let result_value = match t {
        (0, s) => format!("Int: Zero, String: '{}'", s),
        (1, s) => format!("Int: One, String '{}'", s),
        _ => format!("No match!")
    };
    println!("\t{}", result_value);
}

fn match_tuple_string(t: (i32, String)) {
    /* To perform operations on the String component, we need to use the `ref` to borrow
     *    for the `if` clause.
     * We also use a different form of catching in the `match` block, this time allowing
     *    us to report the value of the tuple which went unmatched.
     */
    let result_value = match t {
        (0, s) => format!("Int: Zero, String: '{}'", s),
        (1, ref s) if s == "more words!"=> format!("Int: One, String has too many words!"),
        tt => format!("No match! '{:?}'", tt)
    };
    println!("\t{}", result_value);
}

//endregion