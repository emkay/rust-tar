extern crate tar = "tar";

fn main() {
    let tar = tar::new("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    let data = tar.read();
    println!("{}", data.len());
}
