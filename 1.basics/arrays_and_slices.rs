fn main() {

    // This declares exactly like a list in python, although it's an array!
    let my_array = [10, 20, 30, 40];

    for i in 0..4 {

        let value = my_array[i];
        println!("Value at position [{}] is {}", i, value);

    }

    println!("Array length is {}", my_array.len());

    // While this is possible...
    process_array(my_array);

    /* This won't compile. See the function comments below for why - I guess this does
     *    mean that you can never have an array out of bounds error.
     */
    let my_other_array = [10, 20, 30, 40, 50];
    //process_array(my_other_array);

    /* Using a slice is a much more flexible approach. Since slices still carry their
     *    length, it's easy to use the same iteration syntax when working on a slice.
     * Note that we need to pass by reference when working with slices.
     */
    process_slice(&my_array);
    process_slice(&my_other_array);

    // We can also slice arrays in a very pythonic way:
    let array_slice = &my_array[0..2];

    /* We can't print an array using println!() directly, but there is a
     *    debug print option which achieves the same thing.
     */
    println!("My array was: {:?}", my_array);
    println!("My slice is: {:?}", array_slice);
}

fn process_array(this_array: [i32; 4]) {

    /* This is a pain to do - look at the typing needed - I have to specify the
     *    size as a parameter.
     * This makes the function extremely limited in scope - it will only be able
     *    to accept an array of known size. There are uses for this, of course, but
     *    not for most of my use cases.
     */

    let mut array_sum: i32 = 0;

    for i in 0..4 {
        array_sum += this_array[i];
    }

    println!("process_array :: The sum of the array values is {}", array_sum);
}

fn process_slice(this_array: &[i32]) {

    let mut array_sum: i32 = 0;

    for i in 0..this_array.len() {
        array_sum += this_array[i];
    }

    println!("process_slice :: The sum of the array values is {}", array_sum);

}