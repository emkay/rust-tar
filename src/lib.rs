#![crate_name = "tar"]
#![crate_type = "lib"]
#![license = "MIT"]

use std::io::fs::File;
use std::collections::hashmap::HashMap;

pub fn new(filepath: &'static str) -> Tar {
    let mut fields = HashMap::<&str, uint>::new();
    fields.insert("path", 1);
    fields.insert("mode", 2);
    fields.insert("uid", 3);
    fields.insert("gid", 4);
    fields.insert("size", 5);
    fields.insert("mtime", 6);
    fields.insert("cksum", 7);
    fields.insert("type", 8);
    fields.insert("linkpath", 9);

    Tar {
        filepath: filepath,
        fields: fields
    }
}

struct TarHeader;

impl TarHeader {
    fn decode() {
        fail!();
    }

    fn encode() {
        fail!();
    }

    fn calcSum() {
        fail!();
    }

    fn checkSum() {
        fail!();
    }
}

pub struct Tar {
    filepath: &'static str,
    fields: HashMap<&'static str, uint>
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
