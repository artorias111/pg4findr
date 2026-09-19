//tests/regression.rs

use assert_cmd::Command;
use predicates::str::diff;

#[test]
fn phred33_full_range_output_unchanged() {
    Command::cargo_bin("pg4findr")
        .unwrap()
        .args([
            "--reads",
            "tests/data/phred33_full_range.fastq",
            "--min-score",
            "0",
        ])
        .assert()
        .success()
        .stdout(diff(include_str!("data/phred33_full_range.expected.bed")));
}
