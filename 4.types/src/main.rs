fn main() {

    make_header("CASTING");
    /* Rust does not allow implicit casting (strongly typed), but the `as` keyword can be used
     *    to quickly change a primitive type on the fly.
     */
     let decimal_value: f32 = 6.54321;
     println!("Value as decimal: {}", decimal_value);

     let integer_value = decimal_value as u16;
     println!("Value as integer: {}", integer_value);

     // Be careful about pushing values into smaller sizes...
     let big_number = 1000;
     println!("Big number as u16: {}", big_number);
     println!("Big number as u8: {}", big_number as u8);

     /* That second line prints 232. Why?
      * When pushing into a smaller type, the least significant bits are kept and the
      *    most significant bits are truncated.
      * This means that 1,000 (in binary) is masked as follows:
      * u16 = 0000001111101000 = 1,000
      *  u8 = ********11101000 =   232
      */

    // When going similar things with floats, a `saturating cast` is performed, rather than truncation:
    let big_float = 300.5;
    println!("Big float as f32: {}", big_float);
    println!("Big float as u8: {}", big_float as u8);

    /* That prints as 255, because rather than clipping the MSB positions, `big_float` is just
     *    made into the largest value that can fit into a `u8`.
     */

    make_header("LITERALS");
    // Numeric values can be declared with their type suffixed:
    let first_value: u16 = 1000;
    let second_value = 1000_u16;
    println!("Size of first value: {} bytes", std::mem::size_of_val(&first_value));
    println!("Size of second value: {} bytes", std::mem::size_of_val(&second_value));

    make_header("INFERENCE");
    // The compiler also examines how a variable is used in its life to get hints as to the type
    let elem = 5u8; // elem is a `u8`
    let mut vec = Vec::new(); // vec is a `Vec`

    vec.push(elem); // vec is infered to be a `Vec<u8>`
    println!("{:?}", vec);

    make_header("ALIASING");
    /* The last thing here, the `type` statement can be used to give a new name to a type.
     * The new name must be in upper camel case.
     */
    type SillyTypeName = u16;
    
    let first_integer: u16 = 1000;
    let second_integer: SillyTypeName = 1000;

    // Can remove the camel case requirement if it's really necessary...
    #[allow(non_camel_case_types)]
    type stupid_var_type_name = u16;
    let third_integer: stupid_var_type_name = 1000;

    // These can all be added, because they are still of the same type even if named differently.
    println!("{} + {} + {} = {}", first_integer, second_integer, third_integer, first_integer + second_integer + third_integer);
}

fn make_header(title: &str) -> () {
    println!("\n~~~ {} ~~~", title)
}
