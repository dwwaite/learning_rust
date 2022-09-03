use std::collections::HashSet;

fn main() {

    /* This is pretty brief, as I've already used these a fair bit in project work and their API
     *    is pretty similar to the HashMap.
     * The most important thing to take away here is the amazing `.collect()` function can return
     *    a HashSet without any special modification, as long as it is the return value of a
     *    function that specifies a HashSet return.
     */

    let sentence = "look at all these words there are lots and lots and lots and lots and lots";

    println!("{:?}", sentence);

    let sentence_tokens = make_set(sentence);
    println!("{:?}", sentence_tokens);

    // We can also do the usual set operations:
    let other_sentence = "here are some difference words";
    let other_tokens = make_set(other_sentence);

    let shared_words = sentence_tokens.intersection(&other_tokens);
    println!("{:?}", shared_words);

    /* One thing to catch here is that since the `make_set()` function returns &str, not String,
     *    then the contents of the `shared_words` function are borrowed from `sentence_tokens`.
     * There are two ways around this...
     */

     // First, modify the function to call `to_string()` on the contents of the tokenised input
     let first_set = make_set_string(sentence);
     let second_set = make_set_string(other_sentence);

     // Second, a bit simpler, there's a built in function to do this:
     let shared_words = make_intersection_clone(&first_set, &second_set);
     println!("{:?}", shared_words);
}

fn make_set(input_words: &str) -> HashSet<&str> {
    let unique_words = input_words.split_whitespace().collect();
    unique_words
}

fn make_set_string(input_words: &str) -> HashSet<String> {
    let unique_words = input_words.split_whitespace().map(|x| x.to_string()).collect();
    unique_words
}

fn make_intersection_clone(first_set: &HashSet<String>, second_set: &HashSet<String>) -> HashSet<String> {
    first_set.intersection(second_set).cloned().collect()
}