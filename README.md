# Learning rust!

I started out working in `rust` going by [The Book](https://doc.rust-lang.org/book/), but I found that the structure was hard to follow - too many instances where a concept from many chapters later would be introduced and just glossed over. If I read ahead to get the concept, I then found myself reading content that had come in chapters between where I was and where I had jumped to (understandably).

Trying a new tutorial ([here](https://stevedonovan.github.io/rust-gentle-intro/)) for a more example-driven approach.

## Contents

1. [Basics](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
   * Starting out without `cargo`, instead just using `rustc` to compile simple source code.
   * This is already much nicer for this stage, as it makes stashing these outputs much simpler and keeps `git` cleaner.
   * Covering concepts such as:
     * Macros
       * [hello.rs](1.basics/hello.rs)
     * Assert statement
       * [assert_pass.rs](1.basics/assert_pass.rs)
       * [assert_fail.rs](1.basics/assert_fail.rs)
     * Loops and conditionals
       * [loop_with_conditional.rs](1.basics/loop_with_conditional.rs)
       * [loop_without_ternary.rs](1.basics/loop_without_ternary.rs)
     * Mutability and data types (implicit assignment and explicit casting)
       * [mutability.rs](1.basics/mutability.rs)
       * [casting_values.rs](1.basics/casting_values.rs)
     * Pass-by-reference and dereferencing
       * [dereferencing.rs](1.basics/dereferencing.rs)
     * Imports and using the `rustup doc` command
     * Arrays and slices, and unwrapping `Option` types
       * [arrays_and_slices.rs](1.basics/arrays_and_slices.rs)
       * [slices_and_optionals.rs](1.basics/slices_and_optionals.rs)
       * [vector_manipulation.rs](1.basics/vector_manipulation.rs)
     * Vectors, iterators, and windows/chunks
       * [vectors_and_iterators.rs](1.basics/vectors_and_iterators.rs)
     * Strings
       * [strings.rs](1.basics/strings.rs)
       * [strings_indexing.rs](1.basics/strings_indexing.rs)
       * [strings_functions.rs](1.basics/strings_functions.rs)
     * Parsing command line arguments
       * [command_line_capture.rs](1.basics/1.basics/command_line_capture.rs)

Currently up to [here](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#matching).
