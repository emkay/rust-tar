#![crate_name = "tar"]
#![crate_type = "lib"]
#![license = "MIT"]

use std::io::fs::File;
use std::collections::hashmap::HashMap;

pub struct Tar {
    filepath: &'static str,
    pub fields: HashMap<&'static str, uint>,
    pub field_size: HashMap<uint, uint>
}

pub fn new(filepath: &'static str) -> Tar {
    let mut fields = HashMap::<&str, uint>::new();
    let mut field_size = HashMap::<uint, uint>::new();

    fields.insert("path", 1);
    fields.insert("mode", 2);
    fields.insert("uid", 3);
    fields.insert("gid", 4);
    fields.insert("size", 5);
    fields.insert("mtime", 6);
    fields.insert("cksum", 7);
    fields.insert("type", 8);
    fields.insert("linkpath", 9);

    field_size.insert(*fields.get(&"path"), 100);
    field_size.insert(*fields.get(&"mode"), 8);
    field_size.insert(*fields.get(&"uid"), 8);
    field_size.insert(*fields.get(&"gid"), 8);
    field_size.insert(*fields.get(&"size"), 12);
    field_size.insert(*fields.get(&"mtime"), 12);
    field_size.insert(*fields.get(&"cksum"), 8);
    field_size.insert(*fields.get(&"type"), 1);
    field_size.insert(*fields.get(&"linkpath"), 100);

    Tar {
        filepath: filepath,
        fields: fields,
        field_size: field_size
    }
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

struct TarHeader;

impl TarHeader {
    fn decode() {
        fail!();
    }

    fn encode() {
        fail!();
    }

    fn calc_sum() {
        fail!();
    }

    fn check_sum() {
        fail!();
    }
}
