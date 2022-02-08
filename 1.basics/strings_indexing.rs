fn main() {

    /* Indexing strings in rust is not exactly hard, but there are some considerations to keep in mind.
     * Firstly, strings and string slices can be sliced using indices like so:
     */
    let string_value = "string_slice";

    println!("Slices:");
    println!("    String slice: {:?}", &string_value[1..]);
    //println!("    String real: {:?}", string_value[1..]);

    /* However, we cannot index a string directly. The `string_value` slice above fails without the
     *     borrow operator to coerce a string slice.
     * This is because strings are NOT simply an array of char values
     *    1. Each position in a string is a UTF-8 character
     *    2. Characters can consist of multiple bytes
     * There is not always one-to-one relationship between the length of a string and the contents:
     */
    report_string(&string_value);

    let complex_string = "Hi! ¡Hola! привет!";
    report_string(&complex_string);

    // What this means is that there is a difference between indexing by numbers and by string operations
    let s = "i";
    println!("The first character is {}", &s[0..1]);
    let _s = "¡";
    //println!("The first byte is {}", &_s[0..1]);
    // The line above crashes, as it is accessing the first byte of the char, not the first character.

}

fn report_string(my_string: &str) {
    println!("");
    println!("The string is '{}'", my_string);
    println!("    {} is {} bytes long.", my_string, my_string.len());
    println!("    {} contains {} characters.", my_string, my_string.chars().count());
}