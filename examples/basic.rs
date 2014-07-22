extern crate tar = "tar";

fn main() {
    let data = tar::read("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    println!("{}", data.len());
}
