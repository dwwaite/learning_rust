fn main() {

    // By default, variables are frozen in rust
    let _immutable_binding: u16 = 1;
    let mut _mutable_binding: u16 = 1;

    /* Rust is a strongly typed language, but the compiler can infer casts from assignment.
     * This is not always the case, especially for more complicated structures, but it does
     *    mean that the code above could also be written as:
     */
     let _immutable_binding = 1;
     let mut _mutable_binding = 1;

    /* Variables are scoped by blocks, but due to the borrow checker this is
     *    extremely tight to work with.
     * Observe how this works with shadow binding.
     */

     let shadowed_binding = 1;
     println!("Original value: {}", shadowed_binding);

     {
         let shadowed_binding = "abc";
         println!("Shadowed value inside the block: {}", shadowed_binding);
     }
     println!("Original value again, outside the block: {}", shadowed_binding);

}