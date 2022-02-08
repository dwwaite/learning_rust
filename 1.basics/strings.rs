fn main() {

    /* Strings in rust can be a bit tricky to conceptualise. When we use hardcoded text, it is not parsed
     *    as a string, but instead a slice of a string (&str). For simply printing, this doesn't matter
     *    but if the plan is to manipulate it, then the data must be cast into the string type.
     */
    let static_text = "Hello world"; // &str
    println!("The string slice is: {}", static_text);

    let string_text = static_text.to_string();  // it's now an allocated string
    println!("The value is now a string: {}", string_text);

    // We can coerce a string to a string slice with the borrow operator:
    slice_to_string(static_text);
    slice_to_string(&string_text);

    // We can also push/pop a string, just like any vector
    let mut dynamic_string = String::new();

    for x in string_text.chars() {
        dynamic_string.push(x);
    }

    // There's also this shortcut...
    dynamic_string += "!";

    // ...and a different syntax for pushing a string (versus character):
    dynamic_string.push_str(" 1 2 3.");

    println!("The dynamically pushed string is: {}", dynamic_string);

    // We can also convert arrays of other data types into strings very easily:
    let my_array = [10, 20, 30, 40];
    let array_string = array_into_string(&my_array);
    println!("The string representation of the array is: {:?}", array_string);

}

fn slice_to_string(val: &str) {
    println!("Borrowed value: {}", val);
}

fn array_into_string(int_slice: &[i32]) -> String {

    let mut string_repr = "[".to_string();

    for i_value in int_slice {

        string_repr.push_str(&i_value.to_string());
        string_repr.push_str(", ");
    }

    // Remove the trailing two characters
    string_repr.pop();
    string_repr.pop();

    string_repr += &"]".to_string();
    string_repr
}