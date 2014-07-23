#![crate_name = "tar"]
#![crate_type = "lib"]
#![license = "MIT"]

use std::io::fs::File;

pub fn get_type(s: &'static str) -> &'static str {
    match s {
        "0" => "File",
        "\0" => "OldFile",
        "" => "OldFile",
        "1" => "Link",
        "2" => "SymbolicLink",
        "3" => "CharacterDevice",
        "4" => "BlockDevice",
        "5" => "Directory",
        "6" => "FIFO",
        "7" => "ContiguousFile",
        "g" => "GlobalExtendedHeader",
        "x" => "ExtendedHeader",
        "A" => "SolarisACL",
        "D" => "GNUDumpDir",
        "I" => "Inode",
        "K" => "NextFileHasLongLinkpath",
        "L" => "NextFileHasLongPath",
        "M" => "ContinuationFile",
        "N" => "OldGnuLongPath",
        "S" => "SparseFile",
        "V" => "TapeVolumeHeader",
        "X" => "OldExtendedHeader",
        _ => "Unknown"
    }
}

pub fn get_mode(s: &'static str) -> uint {
   match s {
       "suid" => 04000,
       "sgid" => 02000,
       "svtx" => 01000,
       "uread" => 0400,
       "uwrite" => 0200,
       "uexec" => 0100,
       "gread" => 040,
       "gwrite" => 020,
       "gexec" => 010,
       "oread" => 4,
       "owrite" => 2,
       "oexec" => 1,
       "all" => 07777,
       _ => 0
   }
}


pub fn read(f: &'static str) -> Vec<Vec<u8>> {
    static BLOCK_SIZE: uint = 512;

    let mut split_data = vec!();
    let mut cur_block = 0;
    let mut handle = File::open(&Path::new(f));
    let data = handle.read_to_end().unwrap();

    while cur_block < data.len() {
        split_data.push(Vec::from_slice(data.slice(cur_block, cur_block + BLOCK_SIZE)));
        cur_block += BLOCK_SIZE;
    }
    
    split_data
}
