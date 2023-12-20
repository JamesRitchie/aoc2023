use std::path::PathBuf;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day13::run(input_path, false).unwrap();
    assert_eq!(answer, 405);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day13::run(input_path, true).unwrap();
    assert_eq!(answer, 400);
}
