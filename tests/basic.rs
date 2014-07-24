extern crate tar = "tar";

#[test]
fn test_data_len() {
    let data = tar::read("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    assert_eq!(data.len(), 13);
}

#[test]
fn test_get_type() {
    assert_eq!(tar::get_type("0"), "File");
    assert_eq!(tar::get_type("1"), "Link");
    assert_eq!(tar::get_type("5"), "Directory");
}


#[test]
fn test_get_mode() {
    assert_eq!(tar::get_mode("suid"), 4000);
}
