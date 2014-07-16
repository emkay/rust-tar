use std::io::{BufferedReader, File};

fn read_tar() {
    let file = File::open(&Path::new("tar_test.tar"));
    let mut reader = BufferedReader::new(file);

    let mut buf = [0, ..512];
    match reader.read(buf) {
        Ok(nread) => println!("Read {} bytes", nread),
        Err(e) => println!("Error reading: {}", e),
    }
}

fn main() {
    read_tar();
}
