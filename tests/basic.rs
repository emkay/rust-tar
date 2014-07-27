extern crate tar = "tar";

#[test]
fn test_data_len() {
    let tar = tar::new("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    let data = tar.read();
    assert_eq!(data.len(), 13);
}

#[test]
fn test_fields() {
    let tar = tar::new("/Users/matuzak/workspace/rust-tar/tests/tar_test.tar");
    let path = tar.fields.get(&"path");
    let mode = tar.fields.get(&"mode");
    let uid = tar.fields.get(&"uid");
    let gid = tar.fields.get(&"gid");
    let size = tar.fields.get(&"size");
    let mtime = tar.fields.get(&"mtime");
    let cksum = tar.fields.get(&"cksum");
    let typ = tar.fields.get(&"type");
    let linkpath = tar.fields.get(&"linkpath");


    assert_eq!(path, &1);
    assert_eq!(mode, &2);
    assert_eq!(uid, &3);
    assert_eq!(gid, &4);
    assert_eq!(size, &5);
    assert_eq!(mtime, &6);
    assert_eq!(cksum, &7);
    assert_eq!(typ, &8);
    assert_eq!(linkpath, &9);
}
