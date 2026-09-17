# pg4findr
![CI](https://github.com/artorias111/pg4findr/actions/workflows/ci.yml/badge.svg)


Find G-quadruplex motifs in sequencing reads/genome assemblies. Input is expected to be a fastq (optionally can be gzipped) or a fasta (if you're working with a genome assembly). pg4s are scored according to G4Hunter's scoring algorithm (Bedrat, Lacroix & Mergny 2016, NAR 44:1746-1759, doi 10.1093/nar/gkw006). 

The output is a bed file with the columns sequence_id, start, end, name, bed format score, strand, raw score. Default output is to stdout. See [the BED format specifications](https://github.com/samtools/hts-specs/blob/master/BEDv1.pdf) for more information on the BED file format.


The sequences are found via a regular expression explained in [https://doi.org/10.1093/nar/gki609](https://doi.org/10.1093/nar/gki609) with Rust Regex's `find_iter()` (https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter) to avoid overlaps and repeating counts. 

### Usage
```shell
# The default output is to stdout, you can redirect it to a file. the output is in a standard bed file format.

# quick run with cargo
cargo run -- --reads /path/to/reads.fastq(.gz) > g4_motifs.bed

# Works with multiple read files, and pipe the output to gzip/pigz before saving
cargo run -- --reads ../*.fastq.gz  | pigz > g4_motifs.bed.gz 

# Run on Polar2020
/data2/work/local/pg4findr/pg4findr --reads /path/to/read/or/assembly.fa | pigz > g4_motifs.bed.gz
```

### Example output:

```
read1   9   30  pg4findr_G4_1   428  +  1.7142857142857142
read2   9   30  pg4findr_G4_1   428  -  -1.7142857142857142
```
