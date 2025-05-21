use std::path::PathBuf;

#[test]
fn test_part_one_sample_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_1.txt");
    let answer = day20::run(input_path, false).unwrap();
    assert_eq!(answer, 32000000);
}

#[test]
fn test_part_one_sample_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_2.txt");
    let answer = day20::run(input_path, false).unwrap();
    assert_eq!(answer, 11687500);
}

#[test]
fn test_part_two() {
    // Not a great test case as a chain of flip-flops is just a power of 2, so the answer is just
    // the longest chain.
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_3.txt");
    let answer = day20::run(input_path, true).unwrap();
    assert_eq!(answer, 8);
}
