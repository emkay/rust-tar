extern crate tar = "tar";

#[test]
fn test_data_len() {
    let tar = tar::new("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    let data = tar.read();
    assert_eq!(data.len(), 13);
}
