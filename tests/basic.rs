extern crate tar = "tar";

#[test]
fn test_tar() {
    let data = tar::read("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");

    assert_eq!(data.len(), 13);
}
