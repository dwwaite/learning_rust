fn main() {
    
    /* Vectors as dynamic lists - basically a python List
     * Still must be declared mutable.
     */
    let mut my_vector = Vec::new();
    my_vector.push(10);
    my_vector.push(20);
    my_vector.push(30);

    // Can print everything using the debug operator
    println!("Content of my_vector is {:?}", my_vector);

    // Can either get directly, or via `.get()` and `.unwrap()`
    let index_value = my_vector[0];
    println!("The first value is {} (index)", index_value);

    let get_value = my_vector.get(1).unwrap();
    println!("The second value is {:?} (get operator)", get_value);

    // There's an obvious use case of building a vector, then accessing it as an array...
    let array_vector: &[i32] = &my_vector;
    println!("Array values are {:?}", array_vector);

    // ...or a slice.
    let slice_vector = &my_vector[0..];
    println!("Slice values are {:?}", slice_vector);

    /* A few times now, I've used the iterator syntax of 0..n, which as been super helpful.
     * However, it's a more complex system than just a simple range expansion. Using this syntax
     *    creates a full iterator structure:
     */
     let mut my_iterator = 0..3;
     assert_eq!(my_iterator.next(), Some(0));
     assert_eq!(my_iterator.next().unwrap(), 1); // Same deal, but who wants an Option?
     assert_eq!(my_iterator.next(), Some(2));
     assert_eq!(my_iterator.next(), None);

     /* This is in effect the same as a while loop, but this structure allows rust to optimise heavily
      *    when compiling for production, so is worth using over a more traditional iteration.
      */
      let my_array = [10, 20, 30];
      for i in my_array.iter() { // Explicitly create an iterator
          println!("Array value: {}", i);
      }

      let my_slice = &my_array;
      for i in my_slice { // Implicit creation of iterator
          println!("Slice value: {}", i);
      }

      /* Both of these options are much more efficient forms of iteration than using the syntax:
       *     for i in 0..my_array.len() { my_array[i]; }
       */

    /* The final operations for this section - windowing and chunking.
     *    These are both pre-defined methods for moving a sliding window (windowing) or non-overlapping
     *    window (chunking) over an array.
     */
    let my_array = [10, 20, 30, 40, 50, 60, 70];

    for s in my_array.windows(2) {
        println!("Moving by window; {:?}", s);
    }

    for c in my_array.chunks(2) {
        println!("Moving by chunks; {:?}", c);
    }
}
