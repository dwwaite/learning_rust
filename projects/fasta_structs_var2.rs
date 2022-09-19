use std::io;
use std::env;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashMap;

fn main() {

    // This is not a complete rewrite, as parts o 

    // Capture the user inputs
    let file_path = env::args().nth(1).unwrap();
    let kmer_input = env::args().nth(2).unwrap();

    let kmer_size: usize = kmer_input.parse().expect("Cannot convert value to integer!");

    // Capture the file content
    let file_result = read_file_contents(&file_path);
    let record_vector: Option<Vec::<SeqRecord>> = if file_result.is_ok() {

        // Unpack the results into a vector of Strings (lines), then consume the line content into SeqRecord instances
        let file_content = file_result.unwrap();
        let file_content: Vec<char> = file_content.chars().collect();
        Some(parse_file_to_records(file_content))
    } else {
        None
    };

    if record_vector.is_some() {

        for f in &record_vector.unwrap() {

            let kmer_observations = f.produce_kmer_tally(kmer_size);
            println!("{}", f.name);

            for (kmer, freq) in kmer_observations {
                println!("  K-mer '{}': {}", kmer, freq);
            }
        }
    }
}

//region File processing

fn read_file_contents(file_path: &str)-> io::Result<String> {
    let mut file_handle = File::open(&file_path)?;
    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;
    Ok(file_content)
}

fn parse_file_to_records(file_content: Vec<char>) -> Vec<SeqRecord> {

    // Declare the results vector
    let mut record_vector: Vec<SeqRecord> = Vec::new();

    // Declare variables for tracking the state in the read loop
    let mut is_header: bool = false;
    let mut record_name: Vec<char> = Vec::new();
    let mut record_sequence: Vec<char> = Vec::new();

    // Consume the vector - I don't want it again after this
    for c in file_content {

        if c == '>' {
            // First case, we hit a record delimiter. Store previous record, create new collectors.

            if record_name.len() > 0 {

                let name: String = record_name.iter().collect();
                let new_record = SeqRecord::new(&name, record_sequence);
                record_vector.push(new_record);

                record_name = Vec::new();
                record_sequence = Vec::new();
            }

            is_header = true;
        } else if (c == '\n') && is_header {
            // Second case, we hit a new line and are in header read mode. Swap to sequence read.
            is_header = false;
        } else {
            // Final case, we're adding a character to a vector. Use `is_header` to determine which and make sure a newline is not added

            if is_header {
                record_name.push(c);
            } else { 
                let nt = obtain_nucleotide(c);
                if nt.is_some() {
                    record_sequence.push(nt.unwrap())
                }
            }
        }
    }

    // Catch the final push:
    let name: String = record_name.iter().collect();
    let new_record = SeqRecord::new(&name, record_sequence);
    record_vector.push(new_record);

    record_vector
}

fn obtain_nucleotide(c: char) -> Option<char> {
    // Convert standard nucleotides to upper-case version, or other characters to N.
    match c {
        'a' | 'A' => Some('A'),
        't' | 'T' => Some('T'),
        'g' | 'G' => Some('G'),
        'c' | 'C' => Some('C'),
        '\r' | '\n' => None,
        _ => Some('N')
    }
}

//endregion

// region SeqRecord

#[derive(Debug)]
struct SeqRecord {
    name: String,
    sequence: Vec<char>
}

impl SeqRecord {

    fn new(name: &str, sequence: Vec<char>) -> SeqRecord {
        SeqRecord {
            name: name.to_string(),
            sequence: sequence,
        }
    }

    fn produce_kmer_tally(&self, window_size: usize) -> HashMap<String, i64> {

        // Declare a HashMap to hold the observations
        let mut kmer_observations: HashMap<String, i64> = HashMap::new();

        for kmer_array in self.sequence.windows(window_size) {

            let kmer = String::from_iter(kmer_array);
            *kmer_observations.entry(kmer).or_insert(0) += 1;
        }
        kmer_observations
    }
}

// endregion
