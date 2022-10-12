use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader,Result};

#[derive(Debug)]
pub struct SeqRecord {
    pub name: String,
    pub sequence: Vec<char>
}

impl SeqRecord {

    // Constructor
    pub fn new(name: String, sequence: Vec<char>) -> SeqRecord {
        SeqRecord {
            name: name,
            sequence: sequence
        }
    }

    pub fn produce_kmer_tally(&self, window_size: usize) -> HashMap<String, i64> {

        // Declare a HashMap to hold the observations
        let mut kmer_observations: HashMap<String, i64> = HashMap::new();

        for kmer_array in self.sequence.windows(window_size) {

            let kmer = String::from_iter(kmer_array);
            *kmer_observations.entry(kmer).or_insert(0) += 1;
        }
        kmer_observations
    }
}

pub struct SeqReader {
    file_reader: BufReader<File>
}

impl SeqReader {

    // Constructor
    pub fn new(file_path: &str) -> SeqReader {
        SeqReader {
            file_reader: match parse_file_contents(file_path) {
                Ok(x) => x,
                Err(x) => panic!("{}", x)
            }
        }
    }
}

impl Iterator for SeqReader {
    /* Using the basic Iterator description from the rust docs:
           https://doc.rust-lang.org/rust-by-example/trait/iter.html
     */
    type Item = SeqRecord;

    fn next(&mut self) -> Option<Self::Item> {

        // Read the next section of the file and cast into a vector of characters
        let mut name_buffer: Vec<u8> = Vec::new();
        let mut seq_buffer: Vec<u8> = Vec::new();

        // Convert into the expected data types
        let record_name: String = match self.file_reader.read_until(b'\n', &mut name_buffer) {
            Ok(_) => cast_name_vector(&mut name_buffer),
            Err(_) => panic!("Unable to parse sequence name!")
        };
        let record_seq: Vec<char> = match self.file_reader.read_until(b'>', &mut seq_buffer) {
            Ok(_) => cast_seq_vector(&seq_buffer),
            Err(_) => panic!("Unable to parse sequence content!")
        };

        match seq_buffer.len() {
            0 => None,
            _ => Some(SeqRecord::new(record_name, record_seq))
        }
    }
}

fn parse_file_contents(file_path: &str) -> Result<BufReader<File>> {

    // Try to open a reader on the file path, error out if unable...
    // Check if the file can be read, Error out if not...
    let file_handle = match File::open(&file_path) {
        Ok(x) => x,
        Err(_) => panic!("Unable to open file `{}`!", file_path) // Placeholder for bail! macro...
    };

    // Open a BufReader on the file handle, and read to the first start position
    let mut file_reader = BufReader::new(file_handle);
    let mut temp_buffer: Vec<u8> = Vec::new();
    match file_reader.read_until(b'>', &mut temp_buffer) {
        Ok(_) => (),
        Err(_) => panic!("Unable to read file `{}`!", file_path) // Placeholder for bail! macro...
    }
    Ok(file_reader)
}

fn cast_name_vector(name_buffer: &mut Vec<u8>) -> String {

    // Test for a trailing newline in the vector (u8 = 10)

    /* Older code - essentially index_of() followed by truncation
    let newline_index = match name_buffer.iter().position(|x| *x == 10) {
        Some(x) => x,
        None => name_buffer.len()
    };

    name_buffer.truncate(newline_index);
    */

    // Remove the trailing newline, then map the values to char and return as a String
    name_buffer.pop();
    
    name_buffer.iter()
        .map(|x| *x as char)
        .collect()
}

fn cast_seq_vector(seq_buffer: &Vec<u8>) -> Vec<char> {

    let mut seq_vector: Vec<char> = seq_buffer.iter()
                                        .map(|x| *x as char)
                                        .collect();

    // Purge out unwanted characters - terminal `>` and mid-sequence newlines
    seq_vector.pop();
    seq_vector.retain(|&x| x != '\n');
    seq_vector
}
