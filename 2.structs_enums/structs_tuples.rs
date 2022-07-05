fn main() {

    // Rust has tuples, like any good language:
    println!("Tuples:");
    let return_tuple = differences(10, 5);
    println!("\tFirst value: {}, Second value: {}", return_tuple.0, return_tuple.1);

    let (first_value, second_value) = differences(20, 10);
    println!("\tFirst value: {}, Second value: {}", first_value, second_value);

    let mixed_tuple = ("abc", 15, 1.5);
    assert_eq!(mixed_tuple.0, "abc");
    assert_eq!(mixed_tuple.1, 15);
    assert_eq!(mixed_tuple.2, 1.5);

    /* Tuples are the return type for some of the .iter() functions:
     *    enumerate - like the python functions
     *    zip - same idea as python version, but different implementation
     */
    println!("");
    println!("Iteration:");
    let int_values = [1, 2, 3];
    let str_values = ["One", "Two", "Three"];
    for entry in str_values.iter().enumerate() {
        println!("   Index: {}, Value: {}", entry.0, entry.1);
    }

    for entry in int_values.iter().zip(str_values.iter()) {
        println!("   Zip: {} // {}", entry.0, entry.1);
    }

    /* Structs
     *    Basically, the class equivalent. Really just a tuple with named fields
     */
    println!("");
    println!("Structs:");
    let d = Differences {
        first_value: return_tuple.0,
        second_value: return_tuple.1
    };
    println!("\tFirst value: {}", d.first_value);
    println!("\tSecond value: {}", d.second_value);
}

fn differences(x: i32, y: i32) -> (i32, i32) {
    let xy = x - y;
    let yx = y - x;
    (xy, yx)
}

struct Differences {
    first_value: i32,
    second_value: i32
}