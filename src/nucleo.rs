use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !is_valid_nucleotide(c) {
            return Err(c);
        }
    }

    Ok(dna.chars().filter(|n| *n == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();

    for n in ['A', 'C', 'G', 'T'] {
        counts.insert(n, count(n, dna)?);
    }

    Ok(counts)
}

fn is_valid_nucleotide(n: char) -> bool {
    n == 'A' || n == 'C' || n == 'G' || n == 'T'
}
