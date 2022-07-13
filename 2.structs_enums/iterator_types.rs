fn main() {

    /* We've touch this indirectly, but there are three ways to iteratate over
     *    a vector (or other collection), and each of these have an explicit
     *    and implicit method of invoking.
     */

    let my_array = [
        "a".to_string(),
        "b".to_string(),
        "c".to_string()
    ];

    // Iterate the values as &str
    println!("Print for &str:");
    // Explicit
    for borrowed_s in my_array.iter() {
        print_by_borrow(borrowed_s);
    }
    println!("");

    // Implicit
    for borrowed_s in &my_array {
        print_by_borrow(borrowed_s);
    }
    println!("");

    // Iterate the values as &String
    println!("Print for &String:");
    // Explicit
    for s in my_array.into_iter() {
        print_as_string(&s);
    }
    println!("");

    // Implicit
    for s in my_array {
        print_as_string(&s);
    }
    println!("");

    // Previous method consumed the array, so need to rebuild it, this time as mutable
    let mut my_array = [1, 2, 3];

    // Iterate the values as mutable
    println!("Mutate and print:");
    for mutable_i in my_array.iter_mut() {
        *mutable_i += 1;
        print!("  {},", mutable_i);
    }
    println!("");

    for mutable_i in &mut my_array {
        *mutable_i += 1;
        print!("  {},", mutable_i);
    }
    println!("");
}

fn print_by_borrow(s: &str) {
    print!("  {},", s);
}

fn print_as_string(s: &String) {
    print!("  {},", s);
}
