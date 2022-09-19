from email.policy import default
import sys
from typing import Dict, List
from collections import defaultdict
from dataclasses import dataclass

@dataclass
class SeqRecord():

    name: str = ''
    sequence: str = ''

    def tally_kmer_observations(self, kmer_size: int) -> Dict[str,int]:
        ''' Performing a sliding window over each entry in the contig list, adding to each observation of k-mer. '''

        obs_dict = defaultdict(lambda: 0)

        for i in range(0, len(self.sequence) - kmer_size + 1):
            kmer = self.sequence[i:i+kmer_size]
            obs_dict[kmer] += 1

        return obs_dict

def main():

    # Unpack arguments
    input_file = sys.argv[1]
    kmer_size = parse_kmer_size(sys.argv[2])

    # Parse the sequence content
    record_list = parse_sequence_file(input_file)

    for seq_record in record_list:
        contig_observations = seq_record.tally_kmer_observations(kmer_size)

        print(seq_record.name)
        for kmer, freq in contig_observations.items():
            print(f"  K-mer '{kmer}': {freq}")

#region Functions

def parse_kmer_size(kmer_input: str, kmer_default: int=8) -> int:
    ''' Attempt to parse the user input to an int, returning a default value on failure. '''

    try:

        kmer_value = int(kmer_input)
        return kmer_value

    except:

        print(f"Cannot parse value '{kmer_input}' to an integer, using default value of {kmer_default}")
        return kmer_default

def parse_sequence_file(file_path: str) -> List[SeqRecord]:
    ''' Parse the content of the fasta file, returning a list of the contig sequences. Contig names are discarded. '''

    record_buffer = []
    with open(file_path, 'r') as fna_reader:
        
        for record_chunk in fna_reader.read().split('>')[1:]:

            contig_name, *seq_fragments = record_chunk.split('\n')
            record_buffer.append(
                SeqRecord(name=contig_name, sequence=''.join(seq_fragments).upper())
            )

    return record_buffer

def tally_kmer_observations(contig_list: List[str], kmer_size: int) -> Dict[str,int]:
    ''' Performing a sliding window over each entry in the contig list, adding to each observation of k-mer. '''

    obs_dict = defaultdict(lambda: 0)

    for contig_sequence in contig_list:

        for i in range(0, len(contig_sequence) - kmer_size + 1):

            kmer = contig_sequence[i:i+kmer_size]
            obs_dict[kmer] += 1

    return obs_dict

#endregion

if __name__ == '__main__':
    main()
