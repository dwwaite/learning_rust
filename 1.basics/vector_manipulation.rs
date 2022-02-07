fn main() {

    /* Just a few more notes on vectors. Firstly, there is a macro to shortcut their instantiation, making
     *    them very similar to array creation:
     */
    //let mut my_vector = Vec::new()
    let mut my_vector = vec![10, 20, 30, 40];

    // We obviously know the push operation, and there is of course pop:
    println!("The vector contains: {:?}", my_vector);

    let first_value = my_vector.pop();
    println!("The popped value is: {}", first_value.unwrap());

    println!("The vector now contains: {:?}", my_vector);

    // There is also an extend method, to add a new vector or iterator to an existing one:
    let new_vector = vec![0, 1, 2];
    my_vector.extend(new_vector);
    println!("The extended vector contains: {:?}", my_vector);

    my_vector.extend(0..3);
    println!("The doubly-extended vector contains: {:?}", my_vector);

    // Can assess a vector with a slice - easy.
    assert_eq!(my_vector, &[10, 20, 30, 0, 1, 2, 0, 1, 2]);
}