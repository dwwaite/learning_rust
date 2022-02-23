import sys
from itertools import islice
from typing import Dict, List
from collections import defaultdict

def main():

    # Unpack arguments
    input_file = sys.argv[1]
    kmer_size = parse_kmer_size(sys.argv[2])

    # Parse the sequence content (ignoring contig names)
    contig_list = parse_sequence_file(input_file)

    # Calculate k-mer frequencies
    contig_observations = tally_kmer_observations(contig_list, kmer_size)
    contig_frequencies = summarise_kmer_frequencies(contig_observations)

    # Print results to console
    for kmer, freq in contig_frequencies.items():
        print(f"K-mer '{kmer}': {freq}")

#region Functions

def parse_kmer_size(kmer_input: str, kmer_default: int=8) -> int:
    ''' Attempt to parse the user input to an int, returning a default value on failure. '''

    try:

        kmer_value = int(kmer_input)
        return kmer_value

    except:

        print(f"Cannot parse value '{kmer_input}' to an integer, using default value of {kmer_default}")
        return kmer_default

def parse_sequence_file(file_path: str) -> List[str]:
    ''' Parse the content of the fasta file, returning a list of the contig sequences. Contig names are discarded. '''

    with open(file_path, 'r') as fna_reader:
        
        content = fna_reader.read().split('>')[1:]
        contig_list = [ ''.join(x.split('\n')[1:]) for x in content ]

    return contig_list

def tally_kmer_observations(contig_list: List[str], kmer_size: int) -> Dict[str,int]:
    ''' Performing a sliding window over each entry in the contig list, adding to each observation of k-mer. '''

    obs_dict = defaultdict(lambda: 0)

    for contig_sequence in contig_list:

        contig_iter = iter(contig_sequence)
        result = tuple(islice(contig_iter, kmer_size))

        for elem in contig_iter:

            result = result[1:] + (elem,)
            kmer = ''.join(result)

            obs_dict[kmer] += 1

    return obs_dict

def summarise_kmer_frequencies(contig_obs_dict: Dict[str, int]) -> Dict[str, float]:
    ''' Iterate through the observation tally dictionary and produce a final dictionary of frequencies. '''

    total_kmers = sum([ obs for obs in contig_obs_dict.values() ])
    return { kmer: n_obs / total_kmers for kmer, n_obs in contig_obs_dict.items() }

#endregion

if __name__ == '__main__':
    main()
