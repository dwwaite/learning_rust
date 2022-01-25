fn main() {

    let mut sum = 0 as i32;

    for i in 0..10 {

        increase_by_dereference(&mut sum, i as i32);
        println!("Current value = {}", sum);

    }
}

fn increase_by_dereference(x: &mut i32, i: i32) {

    /* The big thing to note here is that there is no return type - this function
     *    is the equivalent of a `private void` in Java.
     * This implementation requires a number of things to work:
     *    1. The value must be passed in using the & prefix
     *    2. The parameter type here is a) marked as a reference with &, and b) tagged as mutable
     *    3. The value is then dereferenced with the * prefix
     * This is sort of deliberate - it's not impossible to use this approach but explicit return
     *    is favoured, so it's harder to modify values this way.
     *
     */

    *x += i;

}
