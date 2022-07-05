use std::fmt;

fn main() {

    /* Although I say that structs are *like* classes, they are *not* classes.
     *    Structs cannot inherit from other structs, but we can fake this with traits.
     */

    // A simple example, implementing a kind of 'to string' on the i32 type.
    let i_value = 123;
    let s_value = i_value.show();
    assert_eq!(s_value, "123");

    // A better example, implementing the fmt::Debug trait on my Person struct
    let p = Person::new("David", "Waite");
    println!("{:?}", p);

}

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("{}", self)
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

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }

    fn set_first_name(&mut self, new_name: &str) {
        self.first_name = new_name.to_string()
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}