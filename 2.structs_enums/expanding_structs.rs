fn main() {

    // As structs are the class equivalent, there is a ton that can be done to expand them.

    /* To start, the way it was instantiated in the last example is a bit clunky. The
     *    way to approximate a constructor is with the `impl` statement.
     * By convention, `new` is a good keyword to use, but it can be anything:
     */
    let p = Person::new("David", "Waite");
    println!("First name is '{}', second name is '{}'", p.first_name, p.second_name);
    let q = Person::wen("Divad", "Etiaw");
    println!("First name is '{}', second name is '{}'", q.first_name, q.second_name);

    // We can add other functions within the impl statement
    println!("Full name is '{}'", p.full_name());

    let mut p = Person::new("David", "Waite");
    p.set_first_name("William");
    println!("Full name is '{}'", p.full_name());

    let (f, _l) = p.move_to_tuple();
    println!("{}", f);
    //println!("{}", p.first_name); -> Can't do this, the data has now moved.
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

    fn wen(first_name: &str, second_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            second_name: second_name.to_string()
        }
    }

    // Here we use the &self reference, which is basically `self: &Person`
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }

    // Can create setters, although the instance needs to be mutable
    fn set_first_name(&mut self, new_name: &str) {
        self.first_name = new_name.to_string()
    }

    fn move_to_tuple(self) -> (String, String) {
        (self.first_name, self.second_name)
    }

}