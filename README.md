# Learning rust!

I started out working in `rust` going by [The Book](https://doc.rust-lang.org/book/), but I found that the structure was hard to follow - too many instances where a concept from many chapters later would be introduced and just glossed over. If I read ahead to get the concept, I then found myself reading content that had come in chapters between where I was and where I had jumped to (understandably).

Trying a new tutorial ([here](https://stevedonovan.github.io/rust-gentle-intro/)) for a more example-driven approach.

## Tutorials

1. [Basics](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
   * List of what was covered and example code [here](docs/1.basics.md).
1. [Structs, enums, and matching](https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html)
   * List of what was covered and example code [here](docs/2.structs_enums.md).

---

## Test projects

For the completion of every sections of the tutorials, I will create a small project that makes use of the concepts covered. Depending on whether I want a small genome or large one I have two references on hand, *E. coli* and *Rosa*.

```bash
wget -O E_coli.fna.gz https://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/005/845/GCF_000005845.2_ASM584v2/GCF_000005845.2_ASM584v2_genomic.fna.gz
gunzip E_coli.fna.gz

wget -O R_multiflora.fna.gz https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/002/564/525/GCA_002564525.1_RMU_r2.0/GCA_002564525.1_RMU_r2.0_genomic.fna.gz
gunzip R_multiflora.fna.gz

grep -c ">" E_coli.fna
# 1
grep -c ">" R_multiflora.fna
# 83,189
```

#### Basics

At this point all I really know how to do is pass to functions, parse command line arguments, and read files. This is all I need to be able to do to create a *k*-mer frequency calculator for a genome. For interest, I'm creating a few alternatives in `python` to see how speed differs compared with `rust`. As my tutorials have not yet covered `structs`, I will not use classes in the `python` version and will try to keep the level of `python` roughly equivalent to what my `rust` is (i.e. no `argparse`). That said, I will use things like comprehensions, because they're a big part of speeding up `python`.

```bash
time python3 projects/basics.py E_coli.fna 8 > /dev/null
# real    0m27.606s

time python3 projects/basics_optimised.py E_coli.fna 8 > /dev/null
# real    0m1.108s

rustc projects/basics.rs
time ./basics E_coli.fna 8 > /dev/null
# real    0m17.717s

rustc -O projects/basics.rs
time ./basics E_coli.fna 8 > /dev/null
# real    0m1.165s
```

Already my absolute beginner `rust` is more efficient that my `python`. Awesome.

However, there were definitely some teething issues when working here. For example, I have to read the sequences in as a `String`, but then use the window function to slide over a vector of `char`s. I tried to replace the unncessary casting by just reading in as a vector of `char`s from the start but I encountered some errors that I could not solve at my current level. Best to park it as a good first attempt, then continue to learn about `rust` and see if the solutions arise.

---

#### Basics with structs

This is basically an extension on the previous project, making use of the structs and enums tutorials. In this instance, I'm not really going for speed but just trying to implement a sensible fasta struct. I know from reading ahead that the *next* section of the tutorials revisits file reading and I'll probably get some efficiency improvements there, so for now just focus on clean code.

This time around, I want a larger and more fragmented genome. I'm also going to ignore the global *k*-mer frequency calculation - for this implementation there is only a per-contig *k*-mer tally. This will require a new `python` equivalent (`basics_structs.py`) which is based off the `basics_optimised.py` script.

```bash
time python3 projects/basics_structs.py E_coli.fna 8 > /dev/null
# real    0m1.881s

rustc -O projects/basics_structs.rs
time ./basics_structs E_coli.fna 8 > /dev/null
# real    0m0.904s
```

Slight advantage, but there's a fair amount of I/O here, relative to the amount of processing, so try with a larger file to check the computation:

```bash
time python3 projects/basics_structs_char.py R_multiflora.fna 8 > /dev/null
# real    6m9.736s

time ./basics_structs_char R_multiflora.fna 8 > /dev/null
# real    4m24.882s
```

So there's a significant `rust` advantage, not even factoring in the fact that the `python` version uses a much faster file reading method compared with the `rust` version where I read as `char`, cast to `String`, then process as a combination of `Vec<char` and `String`, resulting in a lot of type conversions. The next thing I want to try is to use a different implementation of the `sequence` field of the `SeqRecord` struct. Rather than process it as a `Vec<char>` the whole way through, keep it as a `String` while building, then convert to the `Vec<char>` for computing the *k*-mer tally.

```bash
rustc -O projects/basics_structs_string.rs
time ./basics_structs_string E_coli.fna 8 > /dev/null
# real    0m0.670s

time ./basics_structs_string R_multiflora.fna 8 > /dev/null
# real    4m28.368s
```

Interesting discovery, they're basically equivalent. Really nice to know, because building a `String` is much simpler (tidier) than extending a vector. Will remember this for future work. Finally, I want to try an implementation where everything is read as a single `Vec<char` and then handled as such, only casting off to `String` where necessary.

```bash
rustc -O projects/basics_structs_read.rs
time ./basics_structs_read E_coli.fna 8 > /dev/null
# real    0m0.584s

time ./basics_structs_read R_multiflora.fna 8 > /dev/null
# real    4m45.364s
```

This is actually the worst option, although the code does simplify quite a bit.

---
