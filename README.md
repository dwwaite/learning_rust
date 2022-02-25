# Learning rust!

I started out working in `rust` going by [The Book](https://doc.rust-lang.org/book/), but I found that the structure was hard to follow - too many instances where a concept from many chapters later would be introduced and just glossed over. If I read ahead to get the concept, I then found myself reading content that had come in chapters between where I was and where I had jumped to (understandably).

Trying a new tutorial ([here](https://stevedonovan.github.io/rust-gentle-intro/)) for a more example-driven approach.

## Tutorials

1. [Basics](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
   * List of what was covered and example code [here](docs/1.basics.md).

Currently up to [here](https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html).

---

## Test projects

For the completion of every sections of the tutorials, I will create a small project that makes use of the concepts covered.

#### Basics

At this point all I really know how to do is pass to functions, parse command line arguments, and read files. This is all I need to be able to do to create a *k*-mer frequency calculator for a genome.

Source genome is the canonical *E. coli* K-12 reference, because microbiology.

```bash
wget -O E_coli.fna.gz https://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/005/845/GCF_000005845.2_ASM584v2/GCF_000005845.2_ASM584v2_genomic.fna.gz
gunzip E_coli.fna.gz
```

For interest, I'm creating a few alternatives in `python` to see how speed differs compared with `rust`. As my tutorials have not yet covered `structs`, I will not use classes in the `python` version and will try to keep the level of `python` roughly equivalent to what my `rust` is (i.e. no `argparse`). That said, I will use things like comprehensions, because they're a big part of speeding up `python`.

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

However, there were definitely some teething issues when working here. For example, I have to read the sequences in as `String`s, but then use the window function to slide over a vector of `char`s. I tried to replace the unncessary casting by just reading in as a vector of `char`s from the start but I encountered some errors that I could not solve at my current level. Best to park it as a good first attempt, then continue to learn about `rust` and see if the solutions arise.

---
