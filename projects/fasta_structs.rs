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
        let file_lines: Vec<&str> = file_content.split("\n").collect();
        Some(parse_file_to_records(file_lines))
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

fn check_is_header(file_line: &str) -> bool {

    // Unwrap the value correctly if Some(), else add a poison pill.
    let ch = &file_line.chars().nth(0).unwrap_or('x');

    match ch {
        '>' => true,
        _ => false
    }
}

fn parse_file_to_records(file_lines: Vec<&str>) -> Vec<SeqRecord> {

    // Declare the results vector and the working struct for capturing data
    let mut record_vector: Vec<SeqRecord> = Vec::new();
    let mut current_record = SeqRecord::new("");

    for file_line in &file_lines {

        // Case: Encountered a header
        if check_is_header(file_line) {    

            // If the current fasta has been loaded, store it, then either way create a new empty record
            if current_record.is_loaded() {
                record_vector.push(current_record);
            }
            let sequence_name = &file_line[1..];
            current_record = SeqRecord::new(sequence_name);

        // Case: We are at a sequence line, or the end of the file
        } else {
            current_record.extend_sequence(file_line);
        }
    }

    // Close out the final entry
    if current_record.is_loaded() {
        record_vector.push(current_record);
    }

    record_vector
}


//endregion

// region SeqRecord

#[derive(Debug)]
struct SeqRecord {
    name: String,
    sequence: Vec<char>
}

impl SeqRecord {

    fn new(name: &str) -> SeqRecord {
        SeqRecord {
            name: name.to_string(),
            sequence: Vec::new(),
        }
    }

    fn extend_sequence(&mut self, new_sequence: &str) {

        // Push all sequences characters to uppercase, so there are't mismatches between 'a' and 'A'.
        let converted_nts = &new_sequence.to_uppercase();

        // Specifying the data type on the left hand saves me a turbofish on the right hand.
        let additional_nts: Vec<char> = converted_nts.chars().collect();

        self.sequence.extend(additional_nts);
    }

    fn is_loaded(&self) -> bool {
        if self.name != "".to_string() {
            true
        } else {
            false
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
