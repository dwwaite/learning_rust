// `allow` required to silence warnings because only one variant is used.
#[allow(dead_code)]
enum Colour {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {

    // Create a vector of `Color` values.
    let colour_vector: [Colour; 4] = [ Colour::Red, Colour::Green, Colour::RGB(200, 144, 155), Colour::CMYK(10, 10, 10, 10) ];

    println!("What color is it?");

    // Like C++, but with build in iteration!
    for c in colour_vector.iter() {

        /* An `enum` can be destructured using a `match`.
         * But in the case of the RGB element, the elements can be named within the match
         *    immediate access. This language is so cool!
         */
        match c {
            Colour::Red   => println!("\tThe colour is Red!"),
            Colour::Blue  => println!("\tThe colour is Blue!"),
            Colour::Green => println!("\tThe colour is Green!"),
            Colour::RGB(r, g, b) => println!("\tThe colour is an RGB ({}, {}, {})!", r, g, b),
            _ => println!("\tThe colour is some other fancy-pants colouring scheme!")
        }

    }

    /* More directly, can have have a `use` declaration so that manual scoping
     *    isn't needed.
     */
     use crate::Colour::{Red, Blue};
     match colour_vector[0] {
        Red => println!("Red no-scope!"),
        _ => println!("What am I doing with my life?")
    }
}
