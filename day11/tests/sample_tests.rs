use std::path::PathBuf;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day11::run(input_path, false).unwrap();
    assert_eq!(answer, 374);
}

#[test]
fn test_part_two() {
    // No answer provided for the sample input with the final expansion factor, using the answer
    // produced by code which correctly answered the challenge input.
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day11::run(input_path, true).unwrap();
    assert_eq!(answer, 82000210);
}
