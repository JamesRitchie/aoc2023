use std::path::PathBuf;

#[test]
fn test_part_one_sample_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_1.txt");
    let answer = day10::run(input_path, false).unwrap();
    assert_eq!(answer, 4);
}

#[test]
fn test_part_one_sample_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_2.txt");
    let answer = day10::run(input_path, false).unwrap();
    assert_eq!(answer, 8);
}

#[test]
fn test_part_two_sample_three() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_3.txt");
    let answer = day10::run(input_path, true).unwrap();
    assert_eq!(answer, 4);
}

#[test]
fn test_part_two_sample_four() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_4.txt");
    let answer = day10::run(input_path, true).unwrap();
    assert_eq!(answer, 8);
}

#[test]
fn test_part_two_sample_five() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_5.txt");
    let answer = day10::run(input_path, true).unwrap();
    assert_eq!(answer, 10);
}
