fn main() {

    /* With everything said in the `moving.rs` example, it's important to keep an eye on
     *    the scope of variables. Obviously, since a reference is a pointer to an existing
     *    variable, the reference cannot outlive the original value.
     * In the context of functions, this will never happen. But rust is block-scoped, meaning
     *    that it's possible to create sub-sections of code with new variables inside an
     *    existing block, and this makes it reasonably easy to refer to variables outside of
     *    their scope if not careful.
     */

    let a = 10;
    let b = "abc";
    {
        // a and b are visible here, so is c
        let c = 20;

        println!("Inside the block!");
        println!("    a = {}", a);
        println!("    b = {}", b);
        println!("    c = {}", c);
    }
    // But out here, `c` is no longer available

    // Here, we can create a shadow of an exist variable within a new block
    println!("Blocks and shadows");
    println!("    b = {}", b);
    {
        let b = "def";
        println!("    b = {}", b);

    }
    // But once we're outside, we're using the original `b` again
    println!("    b = {}", b);

    /* This also applies to loops, as these create new blocks as part of their syntax.
     *    In rust, as soon as a variable goes out of scope it is destroyed and all memory
     *    is reclaimed.
     */


}