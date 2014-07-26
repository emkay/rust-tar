extern crate tar = "tar";

#[test]
fn test_data_len() {
    let tar = tar::new("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    let data = tar.read();
    assert_eq!(data.len(), 13);
}

#[test]
fn test_fields() {
    let tar = tar::new("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    let path = tar.fields.get(&"path");

    assert_eq!(path, 1);
}
