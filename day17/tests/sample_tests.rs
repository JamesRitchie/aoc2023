use std::path::PathBuf;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_1.txt");
    let answer = day17::run(input_path, false).unwrap();
    assert_eq!(answer, 102);
}

#[test]
fn test_part_two_sample_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_1.txt");
    let answer = day17::run(input_path, true).unwrap();
    assert_eq!(answer, 94);
}

#[test]
fn test_part_two_sample_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_2.txt");
    let answer = day17::run(input_path, true).unwrap();
    assert_eq!(answer, 71);
}
