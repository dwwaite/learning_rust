use std::env;

fn main() {

    // This is a slight step ahead, just so that I can switch through the cases below
    let args: Vec<String> = env::args().collect();
    let flag_value: i32 = args[1].parse().unwrap();
    
    match flag_value {
        1 => demo_panic(), // 18.1 - panic
        2 => demo_empty(), // 18.2 - Option<T>
        3 => demo_unpack_operator(), // 18.2.1
        _ => ()
    }

}

// 18.1 panic example, the basic unit of a crash
fn demo_panic() -> () {
    dnd_class("druid");
    dnd_class("hunter");
}

fn dnd_class(char_class: &str) {
    // Worst. Class. Ever.
    if char_class == "hunter" { panic!("Noooooooo!!!!"); }
    println!("{}, good choice!", char_class);
}

/* 18.2
 *  Handling an empty value using the Option<T> enum
 *  There are two values it took, either Some(T) or None
 */
fn demo_empty() -> () {

    let first_class = Some("druid");
    let second_class = Some("warlock");

    dnd_class_some(first_class);
    dnd_class_some(second_class);
    dnd_class_some(None);
}

fn dnd_class_some(char_class: Option<&str>) -> () {

    match char_class {
        Some("hunter") => panic!("Noooooooo!!!!"),
        Some(char_class) => println!("{}, good choice!", char_class),
        None => println!("No choice beats a hunter!")
    }
    
}

/* 18.2.1
 *  That last example is quite nice for handling the null case, but is quite verbose
 *     and messy, as variables need to be wrapped in Option<T> up front.
 *  When wrapped as an Option<T>, the value can be accessed with the `?` operator
 *  However, this cannot be done when the function returns nothing.
 */
fn operator_usage(i: u8) -> Option<String> {

    let wrapped = Some(i);
    let raw: u8 = wrapped?;

    let str_val = Some(format!("{}", raw));
    
    // If the last expression on the function lacks a semicolon, it is the return value
    str_val
}

 fn demo_unpack_operator() -> () {

    let x = operator_usage(5);
    println!("{:?}", x);
 }
