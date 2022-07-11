fn main() {

    // Creating enums is simple in rust - easier than python in fact
    let u = Direction::Up;
    println!("Enum representations");
    println!("\tDebug: '{:?}'", u);
    println!("\tTo_string: '{}'", u.as_string());

    // Comparisons
    assert_eq!(u, Direction::Up);
    let d = Direction::Down;
    assert_ne!(u.as_string(), d.as_string());

    println!("Rotating the direction:");
    let mut r = Direction::Up;
    for _ in 0..10 {
        println!("\t{}", r.as_string());
        r = r.next();
    }

    // For the next case, lets look at an enum with different variant types
    let n = Value::Number(2.3);
    let s = Value::Str("two point three".to_string());
    let b = Value::Bool(true);

    println!("");
    println!("Value debug:");
    println!("\tn = '{:?}'", n);
    println!("\ts = '{:?}'", s);
    println!("\tb = '{:?}'", b);

    println!("Value report:");
    for v in [n, s, b] {
        print!("\t");
        Value::report(&v);
    }
}

//region Simple enum

// Just like a struct, declare the values.
// By default, they do not implement Debug or Eq operations - need to add these
#[derive(Debug,PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// ...then implement any functions required
impl Direction {
    fn as_string(&self) -> &str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }

    /* Despite the values being listed in an order in the declaration, enums are not ordered.
    *    This can be added, however. This implementation makes the 'next' be a clockwise rotation:
    */
    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

//endregion

//region Complicated enum

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

impl Value {
    fn report(v: &Value) {
        use Value::*;
        match *v {
            Number(n) => println!("Variant is 'Number', value is '{}'", n),
            // 'ref' required to avoid a move
            Str(ref s) => println!("Variant is 'String', value is '{}'", s),
            Bool(b) => println!("Variant is 'bool', value is '{}'", b)
        }
    }
}

//endregion