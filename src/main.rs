// main.rs
mod cli;
mod g4;
mod scorer;
mod seq;

use clap::Parser;

use cli::Args;
use g4::{Patterns, find_in_record};
use scorer::g4hunter_scorer;

fn main() {
    let args = Args::parse();
    let pats = Patterns::new();

    for file_path in &args.reads {
        let records = match seq::from_path(file_path) {
            Ok(records) => records,
            Err(e) => {
                eprintln!("Error processing the file {}: {}", file_path, e);
                continue;
            }
        };

        for item in records {
            let record = match item {
                Ok(record) => record,
                Err(e) => {
                    eprintln!("Error processing {}: {}", file_path, e);
                    break;
                }
            };

            // naming
            // let mut index: usize = 1; // unique name, starting from 1, and increases by 1 for every G4 entry

            for (index, m) in (1..).zip(find_in_record(&record.id, &record.seq, &pats)) {
                let scoring_slice = &record.seq[m.start..m.end];
                let (g4_hunter_score, scaled_g4_hunter_score) =
                    g4hunter_scorer(scoring_slice.as_bytes());
                println!(
                    "{}\t{}\t{}\tpg4findr_G4_{}\t{}\t{}\t{}",
                    m.seq_id,               // column 1
                    m.start,                // column 2
                    m.end,                  // column 3
                    index,                  // column 4
                    scaled_g4_hunter_score, // column 5
                    m.strand,               // column 6
                    g4_hunter_score         // column 7
                );
            }

            /*
            for m in find_in_record(&record.id, &record.seq, &pats) {
                let scoring_slice = &record.seq[m.start..m.end];
                let (g4_hunter_score, scaled_g4_hunter_score) =
                    g4hunter_scorer(scoring_slice.as_bytes());
                println!(
                    "{}\t{}\t{}\tpg4findr_G4_{}\t{}\t{}\t{}",
                    m.seq_id,               // column 1
                    m.start,                // column 2
                    m.end,                  // column 3
                    index,                  // column 4
                    scaled_g4_hunter_score, // column 5
                    m.strand,               // column 6
                    g4_hunter_score         // column 7
                );
                index += 1;
            }
            */
        }
    }
}
