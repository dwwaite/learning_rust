use clap::Parser;
use seq_record::SeqReader;
mod seq_record;

//region clap implementation

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ClapArgs {

   /// Input file to be parsed for k-mer frequency
   #[clap(short, long)]
   input: String,

   /// K-mer size for calculation (default = 8)
   #[clap(short, long, value_parser, default_value_t=8)]
   kmer: usize,
}

//endregion

//region error_chain implementation

/*
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Conversion(::std::num::ParseIntError);
        }

        errors {
            InvalidString(v: String) {
                display("Bad string provided ({})", v)
            }
        }
    }
}
use errors::*;
*/

//endregion

fn main() {

    // Unpack the user arguments
    let user_args = ClapArgs::parse();

    // Need to capture failures better...
    let file_path = user_args.input;
    let kmer_size = user_args.kmer;

    // Instantiate the SeqReader on the input file, and load the first entry.
    let mut seq_reader = SeqReader::new(&file_path);
    let mut seq_iteration = seq_reader.next();

    // Walk through the iterator, processing each entry
    while seq_iteration.is_some() {

        //println!("{:?}", seq_iteration.unwrap());
        let seq_record = seq_iteration.unwrap();
        let kmer_observations = seq_record.produce_kmer_tally(kmer_size);
        println!("{}", seq_record.name);

        for (kmer, freq) in kmer_observations {
            println!("  K-mer '{}': {}", kmer, freq);
        }

        seq_iteration = seq_reader.next();
    }
}
