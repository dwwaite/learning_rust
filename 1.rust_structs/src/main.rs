/* Structs can be reused as fields of another struct, so the Rectangle struct (class) is
 * really just a wrapper around two Point structs
 */
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    // A rectangle specified by the top left and bottom right corners.
    // Other detail is calculated on demand.
    top_left: Point,
    bottom_right: Point,
}

fn main() {

    // Instantiate the `Points` for a `Rectangle`
    let top_left: Point = Point { x: 10.3, y: 4.1 };
    println!("Top left coordinates: ({}, {})", top_left.x, top_left.y);

    let bottom_right = Point { x: 5.2, y: 1.3 };
    println!("Bottom right coordinates: ({}, {})", bottom_right.x, bottom_right.y);

    /* This isn't necessary here, but a cool feature of Rust is an in-line
     * destructuring of structs into variables
     */
    let Point { x: top_edge, y: left_edge } = top_left;

    let my_rectangle = Rectangle {
        // Struct instantiation is an expression too...
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    report_rectangle(my_rectangle, 7, 7, 8, 5)
}

fn report_rectangle(rect: Rectangle, le: usize, re: usize, te: usize, be: usize) -> () {

    /* Print out the coordinates of supplied rectangle, using some cool padding format.
     * Unit type `()` return is the equivalent of C# void
     */
    println!("");
    println!("My rectangle spans from the following coordinates:");
    println!("Left edge: {n:>width$}", n=rect.top_left.x, width=le);
    println!("Right edge {n:>width$}", n=rect.bottom_right.x, width=re);
    println!("Top edge: {n:>width$}", n=rect.top_left.y, width=te);
    println!("Bottom edge: {n:>width$}", n=rect.bottom_right.y, width=be);
}