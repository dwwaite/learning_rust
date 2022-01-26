fn main() {

    // There is more to know about slices!
    let my_array = [10, 20, 30, 40];
    let array_slice = &my_array[0..2];

    /* Accessing an array out of bounds will fail on compile, but this restriction
     *    does not apply in slices. The safe way to try access when unknown is done
     *    with the `.get()` function.
     */

     let first_slice_value = array_slice.get(0);
     let last_slice_value = array_slice.get( array_slice.len() ); // Off by one

     // However, when we use `.get()` we can no longer println!() the value...
     //println!("The first value is {}", first_slice_value);

     /* This is because the return type of `.get()` is not the direct value, but
      *    the `Option` type.
      * The Option has two values, Some() and None(), which act as a wrapper around
      *    the actual value (should it exist).
      */
     println!("The first Option is {:?}", first_slice_value);
     println!("The n+1 Option is {:?}", last_slice_value);

     // We can get the value by unwrapping the Option like so:
     println!("When unwrapped, the first value is {}", first_slice_value.unwrap());

     // But unwrapping a None() gives a panic:
     //println!("When unwrapped, the None value is {}", last_slice_value.unwrap());

     /* This leads into something I've seen a lot in online `rust` examples, the use
      *    of conditionals to unwrap Options where safe.
      */
    println!("Is the first Option Some()? {}", first_slice_value.is_some());
    println!("Is the first Option None()? {}", first_slice_value.is_none());

    // We can make use of this to provide a safe unwrapping attempt:
    let last_value = if last_slice_value.is_some() {
        // Note the dereference...
        *last_slice_value.unwrap()
    } else {
        -1
    };
    println!("Could I get to the last value? {}", last_value);

    // Which is pretty long-winded. Thankfully, there's the `.unwrap_or()` function!
    // Still requires the dereference...
    let last_value = *last_slice_value.unwrap_or(&-1);
    println!("Could I get to the last value? {}", last_value);
}
