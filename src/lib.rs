#![crate_name = "tar"]
#![crate_type = "lib"]
#![license = "MIT"]

mod tar {
    use std::io::{BufferedReader, File};

    /*
    pub struct FieldSize {
        pub path: uint,
        pub mode: uint,
        pub uid: uint,
        pub gid: uint,
        pub size: uint,
        pub mtime: uint,
        pub cksum: uint,
        pub typee: uint,
        pub linkpath: uint
    }
    */

    static BLOCK_SIZE: uint = 512;

    pub fn read_tar(chan: Sender<Vec<u8>>) {
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
}
