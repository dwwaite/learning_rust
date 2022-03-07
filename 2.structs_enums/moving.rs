fn main() {

    // Consider this code
    //let s1 = "hello dolly".to_string();
    //let s2 = s1;
    //println!("s1 = {}", s1);

    /* It does not compile, with the compiler error
     *    error[E0382]: borrow of moved value: `s1`
     * The short of this is that rust does not allow two pointers to the same variable.
     *    When s2 is declared, the value in s1 is moved to s2 and the s1 variable is no longer valid
     *    (in other languages, you would either get a copy of s1 at s2, or two pointers pointing to 
     *     the same variable).
     *  If we were working with a primitive it would work, because these are simple values and cheap
     *    to create copies:
     */
    let s1 = 1;
    let s2 = s1;
    println!("s1 = {}", s1);

    /* If we go through the full compiler error for the String version, we find the description that
     *    std::string::String does not implement the `Copy` trait. Any variable type that does not
     *    gets moved instead of copied, so they're easy to spot when compiling.
     * When working with Strings, slices (&str) are pointers to the original value, so this is how
     *    we can get pointer-like behaviour with Strings.
     */
    let s1 = "I am a string!";
    let s2 = &s1;
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    /* And this is why we always pass &str into a function, rather than a String - it is robust to
     *    exactly which text type we are passing in:
     */
    print_string(s1);
    print_string(s2);
    print_string(&"abc");
    print_string(&"abc".to_string());
}

fn print_string(s: &str) {
    println!("Value = {}", s);
}