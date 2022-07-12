fn main() {

    // The simplest way to think of these is as a python lambda.
    let f = |x| x * x; // => lambda x: x * x
    assert_eq!(100, f(10));

    // As we did not declare a type on the closure, it is taken from the first use.
    //assert_eq!(100.0, f(10.0));
    let f2 = |x| x * x;
    assert_eq!(100.0, f2(10.0));

    /* Why would we do this instead of just declaring typed functions, or even generic ones?
     * The answer: Closures exist in the same scope as the block that declares them, so we
     *    can do stuff like this:
     */
    let m = 2.0;
    let c = 1.0;
    let lin = |x| m * x + c;
    println!("Regressions: {}, {}", lin(1.0), lin(2.0));

    /* Going further, closures really compile down into a paired struct/impl so each one
     *    declared has a unique type. However, they all have traits in common so it is
     *    possible to write a function that takes a closure as an argument.
     */
    println!("Regression: {}", apply(3.0, lin));

    // Cool? Not quite. Because now rust has moved lin and it no longer exists...
    //apply(3.0, lin); => error[E0308]: expected `()`, found `f64`

    // ...standard rust rules apply. Just borrow the closure if this is needed
    apply_borrow(3.0, &lin);
    apply_borrow(3.0, &lin);
    apply_borrow(3.0, &lin);

    // What if we want our closure to consume a value?
    let x = "David".to_string();
    let c = move || {
        println!("Name: {}", x);
    };

    c();
    //println!("Name: {}", x); => error[E0382]: borrow of moved value: `x`
    // `x` now lives inside the closure `c`...
    c();
}

// Convention is to use F here, rather than the T for types
fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64) -> f64 {
    f(x)
}

fn apply_borrow<F>(x: f64, f: &F) -> f64
where F: Fn(f64) -> f64 {
    f(x)
}