use std::fmt;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

// Basic `From` and `into` implementation
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// More complicated implementation
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {

    // The return type for the TryFrom must be a `Result`
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {

        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

// Implement `fmt::Display` for structs to get an automatic `to_string` function
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number is {}", self.value)
    }
}

fn main() {

    /* Rust has a str and a String data type. They're not the same, although they can
     *    be cast using the `from` statement.
     */
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("'{}' is a str, '{}' is a String cast from the same literal.", my_str, my_string);

    /* A more elegant case, when working with your own struct, is to implement a `from` then
     *    use the `into()` function.
     */
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let num = 5;
    let num_into: Number = num.into();
    println!("My number is {:?}", num_into);

    /* That's a helpful feature, but is pretty likely to crash in practice. This can be
     *    mitigated with the `TryFrom` and `TryInto` libraries.
     */
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));

    // Automatic `to_string` function through a `fmt::Display` implementation
    let num = Number::from(30);
    println!("{}", num.to_string());

    /* In the reverse, can convert a `str` to a different value with the `parse`
     *    function in either regular or turbo-fish ::<> notation
     */
    let _parsed: i32 = "5".parse().unwrap();
    let _turbo_parsed = "10".parse::<i32>().unwrap();
}
