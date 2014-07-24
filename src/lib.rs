#![crate_name = "tar"]
#![crate_type = "lib"]
#![license = "MIT"]

use std::io::fs::File;

pub fn new(filepath: &'static str) -> Tar {
    Tar { filepath: filepath }
}

pub struct Tar {
    filepath: &'static str
}

impl Tar {

    pub fn read(&self) -> Vec<Vec<u8>> {
        static BLOCK_SIZE: uint = 512;

        let mut split_data = vec!();
        let mut cur_block = 0;
        let mut handle = File::open(&Path::new(self.filepath));
        let data = handle.read_to_end().unwrap();

        while cur_block < data.len() {
            split_data.push(Vec::from_slice(data.slice(cur_block, cur_block + BLOCK_SIZE)));
            cur_block += BLOCK_SIZE;
        }

        split_data
    }

    pub fn extract(&self) {
        fail!();
    }
}
