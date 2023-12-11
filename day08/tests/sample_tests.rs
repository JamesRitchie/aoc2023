use std::path::PathBuf;

use day08;

#[test]
fn test_part_one_sample_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_1.txt");
    let answer = day08::run(input_path, false).unwrap();
    assert_eq!(answer, 2);
}

#[test]
fn test_part_one_sample_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_2.txt");
    let answer = day08::run(input_path, false).unwrap();
    assert_eq!(answer, 6);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_3.txt");
    let answer = day08::run(input_path, true).unwrap();
    assert_eq!(answer, 6);
}
