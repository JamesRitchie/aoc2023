use std::path::PathBuf;

use day01;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_part_one.txt");
    let answer = day01::run(input_path, false).unwrap();
    assert_eq!(answer, 142);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_part_two.txt");
    let answer = day01::run(input_path, true).unwrap();
    assert_eq!(answer, 281); 
}