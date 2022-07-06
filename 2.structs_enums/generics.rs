use std::fmt;

fn main() {

    /* Sometimes, we just want a function that can take a value of any type, rather than writing
     *    a function for each data type. Especially when we don't even know what types might be
     *    called.
     */
    let i = 100;
    let f = 100.0;
    let s = "abc";

    report_item(&i);
    report_item(&f);
    report_item(&s);

    // If I create a new struct, I need to implement Debug otherwise it can't pass into `report_item()`
    //let fail_person = Person_nodebug::new("David", "Waite");
    //report_item(&fail_person);
    let success_person = Person::new("David", "Waite");
    report_item(&success_person);

    /* Beyond Debug, it is sometimes necessary to define functions that only work on particular types,
     *    such as mathematical operators.
     */
    square_value(2);
    square_value(2.0);


}

// This function will accept any data type which implements the Debug trait.
fn report_item<T> (value: &T)
where T: std::fmt::Debug {
    println!("The value is '{:?}'", value);
}

/* This approach does not work, because the result of value*value is of type T::Output
 *
 * fn square_value<T> (value: &T) -> T
 * where T: std::ops::Mul {
 *     value * value
 * }
 */

 /* The next approach would be to change the return type to T::Output. This also fails due to the
  *    borrow checker.
  *
  * fn square_value<T> (value: &T) -> T::Output
  * where T: std::ops::Mul {
  *     value * value
  * }
  */
 fn square_value<T> (value: T) -> T::Output
 where T: std::ops::Mul + Copy {
    value * value
 }

//region struct demos

struct Person_nodebug {
    first_name: String,
    second_name: String
}

impl Person_nodebug {
    fn new(first_name: &str, second_name: &str) -> Person_nodebug {
        Person_nodebug {
            first_name: first_name.to_string(),
            second_name: second_name.to_string()
        }
    }
}

struct Person {
    first_name: String,
    second_name: String
}

impl Person {
    fn new(first_name: &str, second_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            second_name: second_name.to_string()
        }
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.second_name)
    }
}

//endregion


