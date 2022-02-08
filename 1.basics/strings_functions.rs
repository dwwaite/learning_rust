fn main() {

    // Useful string functions:

    // There is a find function, because we always need one. The result comes as an Option.
    let complex_string = "Hi! ¡Hola! привет!";
    let first_space = complex_string.find(" ");

    if first_space.is_some() {
        let sliced_string = &complex_string[first_space.unwrap()..];
        println!("{}", sliced_string);
    }

    /* There is also the `split_whitespace()` function, for breaking sentences.
     *    Note that we need to provide a type hint to the compiler here if using the first approach,
     *    but not for the second approach.
     */
    let long_sentence = "the red fox and the lazy dog";
    let sentence_words: Vec<&str> = long_sentence.split_whitespace().collect();
    println!("{:?}", sentence_words);
 
    let mut sentence_words = Vec::new();
    sentence_words.extend(long_sentence.split_whitespace());
    println!("{:?}", sentence_words);
 
    /* This is not string specific, but the first option there uses the `collect()` function.
     *    This is a handy way to...collect... all of the values of an iterator into a vector.
     *    It's about the same as the following in python:
     *        sentence_words = [ x for x in long_sentence.split(' ') ]
     */
    
    /* On the subject of useful iterator functions, we can also use a closure, which is the
     *    rust equivalent of a lambda.
     */

    // For example, this loop...
    let mut stripped = String::new();
    for c in long_sentence.chars() {
        if c != ' ' {
            stripped += &c.to_string();
        }
    }
    println!("{}", stripped);

    // Could instead be written as:
    let stripped: String = long_sentence.chars()
                                        .filter(|ch| ! ch.is_whitespace())
                                        .collect();
    println!("{}", stripped);
}