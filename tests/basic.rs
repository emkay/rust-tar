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

#[test]
fn test_field_sizes() {
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

    let path_size = tar.field_size.get(path);
    let mode_size = tar.field_size.get(mode);
    let uid_size = tar.field_size.get(uid);
    let gid_size = tar.field_size.get(gid);
    let size_size = tar.field_size.get(size);
    let mtime_size = tar.field_size.get(mtime);
    let cksum_size = tar.field_size.get(cksum);
    let type_size = tar.field_size.get(typ);
    let linkpath_size = tar.field_size.get(linkpath);

    assert_eq!(path_size, &100);
    assert_eq!(mode_size, &8);
    assert_eq!(uid_size, &8);
    assert_eq!(gid_size, &8);
    assert_eq!(size_size, &12);
    assert_eq!(mtime_size, &12);
    assert_eq!(cksum_size, &8);
    assert_eq!(type_size, &1);
    assert_eq!(linkpath_size, &100);


}
