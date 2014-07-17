use std::io::{BufferedReader, File};

fn read_tar(chan: Sender<Vec<u8>>) {
    static BLOCK_SIZE: uint = 512;
    let file = File::open(&Path::new("tar_test.tar"));
    let mut reader = BufferedReader::new(file);

    loop {
        let mut buf = vec!();
        match reader.push(BLOCK_SIZE, &mut buf) {
            Ok(_) => chan.send(buf),
            Err(_) => return,
        }

    }
}

fn main() {
    let (chan, port) = channel();
    read_tar(chan);
    println!("{:s}", port.recv().to_string());
}
