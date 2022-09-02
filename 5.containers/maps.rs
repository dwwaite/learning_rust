use std::collections::HashMap;

fn main() {

    // Here's something I work with a lot in python - the dict!
    /* python_dict = {
     *     'one': 'eins',
     *     'two': 'zwei',
     *     'three':, 'drei'
     * }
     */
    let mut rust_map = HashMap::new();
    rust_map.insert("one", "eins");
    rust_map.insert("two", "zwei");
    rust_map.insert("three", "drei");

    // 'two' in python_dict
    assert_eq!(rust_map.contains_key("two"), true);

    // python_dict.get('two', None)
    /* Two differences here. First, since there is a None return option, the return type in rust
     *    is wrapped in Some() when successful. Secondly, the value returned from `.get()` is a
     *    reference to the original value which affects some comparisons and operations on it.
     */
    let content = rust_map.get("two");
    assert_eq! (content, Some(&"zwei"));

    /* What if we want to change it? There's a `.get_mut()` function.
     * The actual mutation I keep in a block - this means that the `content` variable is lost as soon
     *    as we exit the block and there's no dangling mutable reference to a value of the map for the
     *    rest of the programme.
     */
    println!("Mutable accessing:");
    println!("  {:?}", rust_map.get("two"));
    {
        let content = rust_map.get_mut("two").unwrap();
        *content = "asd";
    }
    println!("  {:?}", rust_map.get("two"));

    // Of course, if we have a map, we need ot be able to iterate through the key/value pairs.
    /* for k, v in python_dict.items():
     *    print(f"{k}: {v}")
     */
    println!("Iteration:");
    for (k, v) in rust_map.iter() {
        println!("  {}: {}", k, v);
    }

    // Finally, a working example. How to dynamically update a dict as we iterate through it. 
    /* from collections import defaultdict
     * python_dict = defaultdict(lambda: 0)
     * for val in list_of_words:
     *     python_dict[val] += 1
     */
    let mut rust_map = HashMap::new();
    let list_of_words = ["The", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];

    for word in list_of_words {
        let lower_word = word.to_lowercase();
        let count = rust_map.entry(lower_word).or_insert(0);
        *count += 1;
    }

    println!("Word counter:");
    for (k, v) in rust_map.iter() {
        println!("  {}: {}", k, v);
    }
}