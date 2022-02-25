use std::io;
use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {

    // Capture the user inputs
    let file_path = env::args().nth(1).unwrap();
    let kmer_input = env::args().nth(2).unwrap();

    let kmer_size: usize = kmer_input.parse().expect("Cannot convert value to integer!");

    // Capture the file content
    let file_result = read_file_contents(&file_path);

    if file_result.is_ok() {

        let contig_list = extract_contig_sequences(&file_result.unwrap());

        let kmer_observations = tally_kmer_observations(&contig_list, kmer_size);

        for (kmer, freq) in kmer_observations {
            println!("K-mer '{}': {}", kmer, freq);
        }

    }

}

// region Fasta reading

fn read_file_contents(file_path: &str)-> io::Result<String> {

    let mut file_handle = File::open(&file_path)?;

    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn extract_contig_sequences(input_string: &str) -> Vec<String> {

    let mut contig_list = Vec::new();
    let mut current_contig = String::new();

    let mut on_header: bool = false;

    for c in input_string.chars() {
        // If we are dealing with a header line...
        if c == '>' {
            // ...set the flag...
            on_header = true;
            // ...and cycle the current contig sequence *if* there is any content in it
            if current_contig.len() > 0 {
                contig_list.push(current_contig);
                current_contig = String::new();
            }
        // Otherwise, if we hit the end of the header line set the flag appropriately
        } else if c == '\n' && on_header {
            on_header = false;
        /* Finally, if we know we are not on a header character, store it into the current record
         *    as long as it is not a newline.
         */
        } else if !on_header && c != '\n' {
            current_contig += &c.to_string();
        }
    }

    // Final tidy up - push the final entry and return
    contig_list.push(current_contig);

    contig_list
}

// endregion

// region Kmer frequency calculations

fn tally_kmer_observations(contig_list: &Vec<String>, kmer_size: usize) -> HashMap<String, f64> {

    // Declare the hashmap for return
    let mut kmer_observations: HashMap<String, i64> = HashMap::new();
    let mut n_kmers: i64 = 0;

    for contig_sequence in contig_list {

        let contig_array: Vec<char> = contig_sequence.chars().collect();

        for contig_kmer in contig_array.windows(kmer_size) {
            // Update the total number of k-mers
            n_kmers += 1;
            // Compress the &[char] into a String
            let mut kmer = String::new();
            for c in contig_kmer {
                kmer += &c.to_string();
            }
            // Update the hashmap
            *kmer_observations.entry(kmer).or_insert(0) += 1;
        }

    }

    // Convert the observations to frequencies
    let mut kmer_frequencies: HashMap<String, f64> = HashMap::new();

    for (kmer, obs) in kmer_observations {
        let freq = (obs / n_kmers) as f64;
        kmer_frequencies.insert(kmer, freq);
    }

    kmer_frequencies
}

// endregion
